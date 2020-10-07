use bitflags::bitflags;

use std::{io, mem::MaybeUninit};

use crate::sqe::Sqe;

pub struct IoRing {
    ring: chakra_sys::io_uring,
}

bitflags! {
    #[derive(Default)]
    pub struct Flags: u32 {
        const IORING_SETUP_IOPOLL       = chakra_sys::IORING_SETUP_IOPOLL as u32;
        const IORING_SETUP_SQPOLL       = chakra_sys::IORING_SETUP_SQPOLL as u32;
        const IORING_SETUP_SQ_AFF       = chakra_sys::IORING_SETUP_SQ_AFF as u32;
        const IORING_SETUP_CQSIZE       = chakra_sys::IORING_SETUP_CQSIZE as u32;
        const IORING_SETUP_CLAMP        = chakra_sys::IORING_SETUP_CLAMP as u32;
        const IORING_SETUP_ATTACH_WQ    = chakra_sys::IORING_SETUP_ATTACH_WQ as u32;
        const IORING_SETUP_R_DISABLED   = chakra_sys::IORING_SETUP_R_DISABLED as u32;
    }
}

impl IoRing {
    /// Initialize the io_uring instance
    pub fn init(entries: u32, flags: Flags) -> Result<Self, io::Error> {
        let mut ring = MaybeUninit::uninit();

        let res =
            unsafe { chakra_sys::io_uring_queue_init(entries, ring.as_mut_ptr(), flags.bits()) };

        if res != 0 {
            return Err(io::Error::from_raw_os_error(res));
        }

        Ok(IoRing {
            ring: unsafe { ring.assume_init() },
        })
    }

    pub fn init_params(entries: u32, flags: Flags) -> Result<(Self, IoRingParams), io::Error> {
        let mut ring = MaybeUninit::uninit();
        let params = IoRingParams::new(flags).as_mut_ffi_ptr();

        let res =
            unsafe { chakra_sys::io_uring_queue_init_params(entries, ring.as_mut_ptr(), params) };

        if res != 0 {
            return Err(io::Error::from_raw_os_error(res));
        }

        let mut kernel_params: Option<IoRingParams> = None;

        unsafe {
            if let Some(out) = params.as_mut() {
                kernel_params = Some((*out).into());
            }
        }

        if let Some(user_params) = kernel_params {
            Ok((
                IoRing {
                    ring: unsafe { ring.assume_init() },
                },
                user_params,
            ))
        } else {
            Err(io::Error::from_raw_os_error(0))
        }
    }

    pub fn get_sqe(&mut self) -> Option<Sqe> {
        let sqe_ptr = unsafe { chakra_sys::io_uring_get_sqe(&mut self.ring as *mut _) };

        Sqe::from_raw(sqe_ptr)
    }
}

bitflags! {
    /// FeatureFlags is returned by the kernel when calling `init_params`
    #[derive(Default)]
    pub struct FeatureFlags: u32 {
        const IORING_FEAT_SINGLE_MMAP       = chakra_sys::IORING_FEAT_SINGLE_MMAP as u32;
        const IORING_FEAT_NODROP            = chakra_sys::IORING_FEAT_NODROP as u32;
        const IORING_FEAT_SUBMIT_STABLE     = chakra_sys::IORING_FEAT_SUBMIT_STABLE as u32;
        const IORING_FEAT_RW_CUR_POS        = chakra_sys::IORING_FEAT_RW_CUR_POS as u32;
        const IORING_FEAT_CUR_PERSONALITY   = chakra_sys::IORING_FEAT_CUR_PERSONALITY as u32;
        const IORING_FEAT_FAST_POLL         = chakra_sys::IORING_FEAT_FAST_POLL as u32;
        const IORING_FEAT_POLL_32BITS       = chakra_sys::IORING_FEAT_POLL_32BITS as u32;
        const IORING_FEAT_SQPOLL_NONFIXED   = chakra_sys::IORING_FEAT_SQPOLL_NONFIXED as u32;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IoRingParams {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: Flags,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub features: FeatureFlags,
    pub wq_fd: u32,
    pub resv: [u32; 3],
    pub sq_off: IoSqringOffsets,
    pub cq_off: IoCqringOffsets,
}

impl IoRingParams {
    pub fn new(flags: Flags) -> Self {
        Self {
            flags,
            sq_entries: 0,
            cq_entries: 0,
            sq_thread_cpu: 0,
            sq_thread_idle: 0,
            features: FeatureFlags::empty(),
            wq_fd: 0,
            resv: [0u32; 3],
            sq_off: IoSqringOffsets::new(),
            cq_off: IoCqringOffsets::new(),
        }
    }

    fn as_mut_ffi_ptr(&mut self) -> *mut chakra_sys::io_uring_params {
        &mut From::from(*self) as *mut _
    }
}

impl From<IoRingParams> for chakra_sys::io_uring_params {
    fn from(params: IoRingParams) -> Self {
        let IoRingParams {
            sq_entries,
            cq_entries,
            flags,
            sq_thread_cpu,
            sq_thread_idle,
            features,
            wq_fd,
            resv,
            sq_off,
            cq_off,
        } = params;

        chakra_sys::io_uring_params {
            sq_entries,
            cq_entries,
            flags: flags.bits(),
            sq_thread_cpu,
            sq_thread_idle,
            features: features.bits(),
            wq_fd,
            resv,
            sq_off: sq_off.into(),
            cq_off: cq_off.into(),
        }
    }
}

impl From<chakra_sys::io_uring_params> for IoRingParams {
    fn from(params: chakra_sys::io_uring_params) -> Self {
        let chakra_sys::io_uring_params {
            sq_entries,
            cq_entries,
            flags,
            sq_thread_cpu,
            sq_thread_idle,
            features,
            wq_fd,
            resv,
            sq_off,
            cq_off,
        } = params;

        IoRingParams {
            sq_entries,
            cq_entries,
            flags: Flags::from_bits_truncate(flags),
            sq_thread_cpu,
            sq_thread_idle,
            features: FeatureFlags::from_bits_truncate(features),
            wq_fd,
            resv,
            sq_off: sq_off.into(),
            cq_off: cq_off.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IoSqringOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

impl IoSqringOffsets {
    fn new() -> Self {
        IoSqringOffsets {
            head: 0,
            tail: 0,
            ring_mask: 0,
            ring_entries: 0,
            flags: 0,
            dropped: 0,
            array: 0,
            resv1: 0,
            resv2: 0,
        }
    }
}

impl From<IoSqringOffsets> for chakra_sys::io_sqring_offsets {
    fn from(off: IoSqringOffsets) -> Self {
        let IoSqringOffsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            flags,
            dropped,
            array,
            resv1,
            resv2,
        } = off;

        chakra_sys::io_sqring_offsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            flags,
            dropped,
            array,
            resv1,
            resv2,
        }
    }
}

impl From<chakra_sys::io_sqring_offsets> for IoSqringOffsets {
    fn from(off: chakra_sys::io_sqring_offsets) -> Self {
        let chakra_sys::io_sqring_offsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            flags,
            dropped,
            array,
            resv1,
            resv2,
        } = off;

        IoSqringOffsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            flags,
            dropped,
            array,
            resv1,
            resv2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IoCqringOffsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub flags: u32,
    pub resv1: u32,
    pub resv2: u64,
}

impl IoCqringOffsets {
    fn new() -> Self {
        IoCqringOffsets {
            head: 0,
            tail: 0,
            ring_mask: 0,
            ring_entries: 0,
            overflow: 0,
            cqes: 0,
            flags: 0,
            resv1: 0,
            resv2: 0,
        }
    }
}

impl From<IoCqringOffsets> for chakra_sys::io_cqring_offsets {
    fn from(off: IoCqringOffsets) -> Self {
        let IoCqringOffsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            overflow,
            cqes,
            flags,
            resv1,
            resv2,
        } = off;

        chakra_sys::io_cqring_offsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            overflow,
            cqes,
            flags,
            resv1,
            resv2,
        }
    }
}

impl From<chakra_sys::io_cqring_offsets> for IoCqringOffsets {
    fn from(off: chakra_sys::io_cqring_offsets) -> Self {
        let chakra_sys::io_cqring_offsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            overflow,
            cqes,
            flags,
            resv1,
            resv2,
        } = off;

        IoCqringOffsets {
            head,
            tail,
            ring_mask,
            ring_entries,
            overflow,
            cqes,
            flags,
            resv1,
            resv2,
        }
    }
}
