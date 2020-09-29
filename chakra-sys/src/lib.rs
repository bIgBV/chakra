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
#[derive(Copy, Clone, Debug)]
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
#[derive(Debug)]
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

// Filled with the offset for mmap(2)
#[repr(C)]
#[derive(Debug)]
pub struct io_sqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub flags: libc::__u32,
    pub dropped: libc::__u32,
    pub array: libc::__u32,
    pub resv1: libc::__u32,
    pub resv2: libc::__u64,
}

// sq_ring->flags
// Needs io_uring_enter to wakeup
pub const IORING_SQ_NEED_WAKEUP: libc::c_uint = 1 << 0;
// CQ ring is overflown
pub const IORING_SQ_CQ_OVERFLOW: libc::c_uint = 1 << 1;

// cq_ring->flags
#[repr(C)]
#[derive(Debug)]
pub struct io_cqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub overflow: libc::__u32,
    pub cqes: libc::__u32,
    pub flags: libc::__u32,
    pub resv1: libc::__u32,
    pub resv2: libc::__u64,
}

// cq_ring->flags
// disable eventfd notifications
pub const IORING_CQ_EVENTFD_DISABLED: libc::c_uint = 1 << 0;

// io_uring_enter flags
pub const IORING_ENTER_GETEVENTS: libc::c_uint = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP: libc::c_uint = 1 << 1;
pub const IORING_ENTER_SQ_WAIT: libc::c_uint = 1 << 2;

/// Parameters passed in for io_uring_setup(2). Copied back with updated info
/// on success
#[repr(C)]
#[derive(Debug)]
pub struct io_uring_params {
    pub sq_entries: libc::__u32,
    pub cq_entries: libc::__u32,
    pub flags: libc::__u32,
    pub sq_thread_cpu: libc::__u32,
    pub sq_thread_idle: libc::__u32,
    pub features: libc::__u32,
    pub wq_fd: libc::__u32,
    pub resv: [libc::__u32; 3],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets
}

// io_uring_params->features flags
pub const IORING_FEAT_SINGLE_MMAP: libc::c_uint = 1 << 0;
pub const IORING_FEAT_NODROP: libc::c_uint = 1 << 1;
pub const IORING_FEAT_SUBMIT_STABLE: libc::c_uint = 1 << 2;
pub const IORING_FEAT_RW_CUR_POS: libc::c_uint = 1 << 3;
pub const IORING_FEAT_CUR_PERSONALITY: libc::c_uint = 1 << 4;
pub const IORING_FEAT_FAST_POLL: libc::c_uint = 1 << 5;
pub const IORING_FEAT_POLL_32BITS: libc::c_uint = 1 << 6;
pub const IORING_FEAT_SQPOLL_NONFIXED: libc::c_uint = 1 << 7;

// io_uring_register(2) opcodes and arguments
pub const IORING_REGISTER_BUFFERS: libc::c_uint = 0;
pub const IORING_UNREGISTER_BUFFERS: libc::c_uint = 1;
pub const IORING_REGISTER_FILES: libc::c_uint = 2;
pub const IORING_UNREGISTER_FILES: libc::c_uint = 3;
pub const IORING_REGISTER_EVENTFD: libc::c_uint = 4;
pub const IORING_UNREGISTER_EVENTFD: libc::c_uint = 5;
pub const IORING_REGISTER_FILES_UPDATE: libc::c_uint = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: libc::c_uint = 7;
pub const IORING_REGISTER_PROBE: libc::c_uint = 8;
pub const IORING_REGISTER_PERSONALITY: libc::c_uint = 9;
pub const IORING_UNREGISTER_PERSONALITY: libc::c_uint = 10;
pub const IORING_REGISTER_RESTRICTIONS: libc::c_uint = 11;
pub const IORING_REGISTER_ENABLE_RINGS: libc::c_uint = 12;

pub const IO_URING_OP_SUPPORTED: libc::c_uint = 1 << 0;

#[repr(C)]
#[derive(Debug)]
pub struct io_uring_probe_op {
    pub op: libc::__u8,
    pub resv: libc::__u8,
    pub flags: libc::__u16,
    pub resv2: libc::__u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct io_uring_probe {
    pub last_op: libc::__u8,
    pub ops_len: libc::__u8,
    pub resv: libc::__u16,
    pub resv2: [libc::__u32; 3],
    pub ops: [io_uring_probe_op; 0],
}

#[repr(C)]
pub struct io_uring_restriction {
    pub opcode: libc::__u16,
    pub restriction_op: restriction_op,
    pub resv: libc::__u8,
    pub resv2: [libc::__u32; 3],
}

#[repr(C)]
pub union restriction_op {
    pub register_op: libc::__u8,
    pub sqe_op: libc::__u8,
    pub sqe_flags: libc::__u8,
}

// io_uring_restriction->opcode values
// allow an io_uring_register(2) opcode
pub const IORING_RESTRICTION_REGISTER_OP: libc::c_uint = 0;
// Allow an sqe opcode
pub const IORING_RESTRICTION_SQE_OP: libc::c_uint = 1;
// Allow sqe flags
pub const IORING_RESTRICTION_SQE_FLAGS_ALLOWED: libc::c_uint = 2;
// Require sqe flags (these flags must be set on each submission)
pub const IORING_RESTRICTION_SQE_FLAGS_REQUIRED: libc::c_uint = 3;


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
