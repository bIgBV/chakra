#[repr(C)]
pub struct io_uring;

#[repr(C)]
pub struct io_uring_sqe {
    /// Operation code for this particular request
    pub opcode: libc::__u8,
    /// Modifier flags common across command types
    pub flags: libc::__u8,
    /// Priority of the request, as defined by `ioprio_set(2)`
    pub ioprio: libc::__u16,
    /// File descriptor associated with the request
    pub fd: libc::__s32,
    /// Offset into a file
    pub file_off: file_off,
    /// Pointer to a buffer or iovecs
    pub addr_off: add_off,
    /// Buffer size or number of iovecs
    pub len: libc::__u32,
    pub cmd_flags: cmd_flags,
    /// Userdata which is copied from SQE into CQE
    pub user_data: libc::__u64,
    pub buf_index_padding: buf_index_padding
}


#[repr(C)]
pub union file_off {
    pub off: libc::__u64,
    pub addr2: libc::__u64
}

#[repr(C)]
pub union add_off {
    pub addr: libc::__u64,
    pub splice_off_in: libc::__u64
}

#[repr(C)]
pub union cmd_flags {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: libc::__u32,
    pub poll_events: libc::__u16,
    pub poll32_events: libc::__u32,
    pub sync_range_flags: libc::__u32,
    pub msg_flags: libc::__u32,
    pub timeout_flags: libc::__u32,
    pub accept_flags: libc::__u32,
    pub cancel_flags: libc::__u32,
    pub open_flags: libc::__u32,
    pub statx_flags: libc::__u32,
    pub fadvise_advice: libc::__u32,
    pub splice_flags: libc::__u32,
}

#[allow(non_camel_case_types)]
type __kernel_rwf_t = libc::c_int;

#[repr(C)]
pub union buf_index_padding {
    pub personality: personality,
    pub pad2: [libc::__u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct personality {
    pub buf_or_group: libc::__u16,
    pub personality: libc::__u16,
    pub splice_fd_in: libc::__s32
}

// sqe->flags
/// Use fixed fleset
pub const IOSQE_FIXED_FILE: libc::__u8 = 1 << 0;
/// Issue after inflight IO
pub const IOSQE_IO_DRAIN: libc::__u8 = 1 << 1;
/// Links next SQW
pub const IOSQE_IO_LINK: libc::__u8 = 1 << 2;
/// like LINK, but stronger
pub const IOSQE_IO_HARDLINK: libc::__u8 = 1 << 3;
/// always go async
pub const IOSQE_ASYNC: libc::__u8 = 1 << 4;
/// select buffer from sqe->buf_group
pub const IOSQE_BUFFER_SELECT: libc::__u8 = 1 << 5;


// io_uring_setup flags
/// io_context is polled
pub const IORING_SETUP_IOPOLL: libc::__u8 = 1 << 0;
/// SQ poll thread
pub const IORING_SETUP_SQPOLL: libc::__u8 = 1 << 1;
/// sq_thread_cpu is valid
pub const IORING_SETUP_SQ_AFF: libc::__u8 = 1 << 2;
/// app defined CQ size
pub const IORING_SETUP_CQSIZE: libc::__u8 = 1 << 3;
/// clamp SQ/CQ ring sizes
pub const IORING_SETUP_CLAMP: libc::__u8 = 1 << 4;
/// attach to existing wq
pub const IORING_SETUP_ATTACH_WQ: libc::__u8 = 1 << 5;
/// start with ring disabled
pub const IORING_SETUP_R_DISABLED: libc::__u8 = 1 << 6;

#[repr(C)]
#[non_exhaustive]
#[allow(nonstandard_style)]
#[derive(Debug)]
pub enum IoUringOp {
	IORING_OP_NOP,
	IORING_OP_READV,
	IORING_OP_WRITEV,
	IORING_OP_FSYNC,
	IORING_OP_READ_FIXED,
	IORING_OP_WRITE_FIXED,
	IORING_OP_POLL_ADD,
	IORING_OP_POLL_REMOVE,
	IORING_OP_SYNC_FILE_RANGE,
	IORING_OP_SENDMSG,
	IORING_OP_RECVMSG,
	IORING_OP_TIMEOUT,
	IORING_OP_TIMEOUT_REMOVE,
	IORING_OP_ACCEPT,
	IORING_OP_ASYNC_CANCEL,
	IORING_OP_LINK_TIMEOUT,
	IORING_OP_CONNECT,
	IORING_OP_FALLOCATE,
	IORING_OP_OPENAT,
	IORING_OP_CLOSE,
	IORING_OP_FILES_UPDATE,
	IORING_OP_STATX,
	IORING_OP_READ,
	IORING_OP_WRITE,
	IORING_OP_FADVISE,
	IORING_OP_MADVISE,
	IORING_OP_SEND,
	IORING_OP_RECV,
	IORING_OP_OPENAT2,
	IORING_OP_EPOLL_CTL,
	IORING_OP_SPLICE,
	IORING_OP_PROVIDE_BUFFERS,
	IORING_OP_REMOVE_BUFFERS,
	IORING_OP_TEE,
	IORING_OP_SHUTDOWN,

	/* this goes last, obviously */
	IORING_OP_LAST,
}

/// sqe->fsync_flags
pub const IORING_FSYNC_DATASYNC: libc::__u32 = 1 << 0;

/// sqe->timeout_flags
pub const IORING_TIMEOUT_ABS: libc::__u32 = 1 << 0;

/// sqe->splice_flags, extends splice(2) flags
pub const SPLICE_F_FD_IN_FIXED: libc::__u32 = 1 << 31;

/// A Completion Queue Event.
#[repr(C)]
pub struct io_uring_cqe {
    user_data: libc::__u64,
    res: libc::__s32,
    flags: libc::__u32
}

/// cqe->flags
/// IORING_CQE_F_BUFFER if set, the upper 16 bits are the buffer ID
pub const IORING_CQE_F_BUFFER: libc::__u32 = 1 << 0;
pub const IORING_CQE_BUFFER_SHIFT: libc::__u32 = 16;

// Magic offsets for the application to mmap the data it needs
pub const IORING_OFF_SQ_RING: libc::__u64 = 0;
pub const IORING_OFF_CQ_RING: libc::__u64 = 0x8000000;
pub const IORING_OFF_SQES: libc::__u64 = 0x10000000;

#[link(name = "uring")]
extern "C" {
    pub fn io_uring_queue_init(entries: libc::c_uint, ring: *mut io_uring, flags: libc::c_uint) -> libc::c_uint;
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
