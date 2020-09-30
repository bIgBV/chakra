#[cfg(target_os = "unix")]
use std::os::unix::AsRawFd;

#[cfg(target_os = "unix")]
use std::{io, mem::MaybeUninit, ptr::NonNull};

#[cfg(target_os = "unix")]
pub struct IoRing {
    ring: chakra_sys::io_uring,
}

#[cfg(target_os = "unix")]
impl IoRing {
    /// Initialize the io_uring instance
    pub fn init(entries: u32, flags: u32) -> Result<Self, io::Error> {
        let mut ring = MaybeUninit::uninit();

        let res = unsafe { chakra_sys::io_uring_queue_init(entries, ring.as_mut_ptr(), flags) };

        if res != 0 {
            return Err(io::Error::from_raw_os_error(res));
        }

        Ok(IoRing {
            ring: unsafe { ring.assume_init() },
        })
    }

    pub fn get_sqe(&mut self) -> Option<Sqe> {
        let sqe_ptr = unsafe { chakra_sys::io_uring_get_sqe(&mut self.ring as *mut _) };

        NonNull::new(sqe_ptr).and_then(|sqe| Some(Sqe { sqe }))
    }
}

#[cfg(target_os = "unix")]
pub struct Sqe {
    sqe: NonNull<chakra_sys::io_uring_sqe>,
}

#[cfg(target_os = "unix")]
impl Sqe {
    pub fn prep_read<T>(&mut self, io: T, buf: &[u8], offset: usize) where T: AsRawFd {
        let fd = io.as_raw_fd();

        let iovec = libc::iovec {
            iov_base: buf.as_mut_ptr() as _,
            iov_len: buf.len()
        };

        unsafe {
            chakra_sys::io_uring_prep_readv(sqe.as_ptr(), fd, &iovec as *const _, 1, offset);
        }
    }
}
