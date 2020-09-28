use libc::c_uint;

#[repr(C)]
pub struct io_uring;

#[link(name = "uring")]
extern "C" {
    pub fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_uint;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ring = io_uring;
        unsafe {
            io_uring_queue_init(256, &mut ring as *mut _, 0);
        }
    }
}
