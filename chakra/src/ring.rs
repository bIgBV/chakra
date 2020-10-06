use bitflags::bitflags;

use std::{io, mem::MaybeUninit, ptr::NonNull};

use crate::sqe::Sqe;

pub struct IoRing {
    ring: chakra_sys::io_uring,
}

bitflags! {
    #[derive(Default)]
    pub struct Flags: u32 {
        const IORING_SETUP_IOPOLL       = chakra_sys::IOURING_SETUP_IOPOLL as u32;
        const IORING_SETUP_SQPOLL       = chakra_sys::IOURING_SETUP_SQPOLL as u32;
        const IORING_SETUP_SQ_AFF       = chakra_sys::IOURING_SETUP_SQ_AFF as u32;
        const IORING_SETUP_CQSIZE       = chakra_sys::IOURING_SETUP_CQSIZE as u32;
        const IORING_SETUP_CLAMP        = chakra_sys::IOURING_SETUP_CLAMP as u32;
        const IORING_SETUP_ATTACH_WQ    = chakra_sys::IOURING_SETUP_ATTACH_WQ as u32;
        const IORING_SETUP_R_DISABLED   = chakra_sys::IOURING_SETUP_R_DISABLED as u32;
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

    pub fn init_params(entries: u32, flags: Flags) -> Result<Self, io::Error> {
        let mut ring = MaybeUninit::uninit();
        let mut params = MaybeUninit::uninit();
    }

    pub fn get_sqe(&mut self) -> Option<Sqe> {
        let sqe_ptr = unsafe { chakra_sys::io_uring_get_sqe(&mut self.ring as *mut _) };

        NonNull::new(sqe_ptr).and_then(|sqe| Some(Sqe { sqe }))
    }
}

bitflags! {
    #[derive(Default)]
    pub struct FeatureFlags: u32 {
        const IORING_FEAT_SINGLE_MMAP       = IORING_FEAT_SINGLE_MMAP as u32;
        const IORING_FEAT_NODROP            = IORING_FEAT_NODROP as u32;
        const IORING_FEAT_SUBMIT_STABLE     = IORING_FEAT_SUBMIT_STABLE as u32;
        const IORING_FEAT_RW_CUR_POS        = IORING_FEAT_RW_CUR_POS as u32;
        const IORING_FEAT_CUR_PERSONALITY   = IORING_FEAT_CUR_PERSONALITY as u32;
        const IORING_FEAT_FAST_POLL         = IORING_FEAT_FAST_POLL as u32;
        const IORING_FEAT_POLL_32BITS       = IORING_FEAT_POLL_32BITS as u32;
        const IORING_FEAT_SQPOLL_NONFIXED   = IORING_FEAT_SQPOLL_NONFIXED as u32;
    }
}

pub struct IORingParams {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: Flags,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub features: FeatureFlags,
    pub wq_fd: u32,
    pub resv: [u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

impl From<IORingParams> for io_uring_params {
    fn from(params: IORingParams) -> Self {
        IORingParams {
            sq_entries,
            cq_entries,
            flags,
            sq_thread_cpu,
            sq_thread_idle,
            features,
            sq_fd,
            resv,
            sq_off,
            cq_off,
        } = params;

        io_uring_params {
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

impl From<IoSqringOffsets> for io_sqring_offsets {
    fn from(off: IoSqringOffsets) -> Self {
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
        } = off;

        io_sqring_offsets {
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

impl From<IoCqringOffsets> for io_cqring_offsets {
    fn from(off: IoCqringOffsets) -> io_cqring_offsets {
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
        } = off;

        io_cqring_offsets {
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
