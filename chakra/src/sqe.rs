use std::{convert::TryInto, os::unix::io::AsRawFd, ptr::NonNull};

pub struct Sqe {
    sqe: NonNull<chakra_sys::io_uring_sqe>,
}

impl Sqe {
    pub fn from_raw(sqe_ptr: *mut chakra_sys::io_uring_sqe) -> Option<Self> {
        NonNull::new(sqe_ptr).and_then(|sqe| Some(Sqe { sqe }))
    }

    pub fn prep_read<T>(&mut self, io: T, buf: &mut [u8], offset: usize)
    where
        T: AsRawFd,
    {
        let fd = io.as_raw_fd();

        let iovec = libc::iovec {
            iov_base: buf.as_mut_ptr() as _,
            iov_len: buf.len(),
        };

        unsafe {
            chakra_sys::io_uring_prep_readv(
                self.sqe.as_ptr(),
                fd,
                &iovec as *const _,
                1,
                offset.try_into().unwrap(),
            );
        }
    }
}
