pub(crate) mod poll_fd;
#[cfg(not(windows))]
pub(crate) mod types;

#[cfg_attr(windows, path = "windows_syscalls.rs")]
pub(crate) mod syscalls;

#[cfg(all(feature = "alloc", linux_kernel))]
pub mod epoll;
