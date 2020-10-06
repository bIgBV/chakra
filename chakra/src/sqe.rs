#[cfg(target_os = "unix")]
use std::os::unix::AsRawFd;
use std::ptr::NonNull;

pub struct Sqe {
    sqe: NonNull<chakra_sys::io_uring_sqe>,
}

impl Sqe {
    pub fn prep_read<T>(&mut self, io: T, buf: &[u8], offset: usize)
    where
        T: AsRawFd,
    {
        let fd = io.as_raw_fd();

        let iovec = libc::iovec {
            iov_base: buf.as_mut_ptr() as _,
            iov_len: buf.len(),
        };

        unsafe {
            chakra_sys::io_uring_prep_readv(sqe.as_ptr(), fd, &iovec as *const _, 1, offset);
        }
    }
}
