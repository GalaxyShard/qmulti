use std::sync::Mutex;

use crate::bindings::*;

use crate::ServiceInfo;

pub(crate) mod register;
pub(crate) mod browse;
pub(crate) mod resolve;

#[derive(Debug)]
struct BonjourFoundService {
    info: ServiceInfo,
}
#[derive(Debug)]
struct BonjourLostService {
    info: ServiceInfo,
}

// https://developer.apple.com/library/archive/documentation/Networking/Conceptual/dns_discovery_api/Introduction.html

struct OwnedDnsService(DNSServiceRef);
unsafe impl Send for OwnedDnsService {}
impl OwnedDnsService {
    fn block_until_handled(&self) -> DNSServiceErrorType {
        assert!(self.is_valid());
        // SAFETY: Must be called with a valid DNSServiceRef; checked for in line above
        unsafe { DNSServiceProcessResult(self.0) }
    }
    fn internal_socket(&self) -> dnssd_sock_t {
        assert!(self.is_valid());

        // SAFETY: Must be called with a valid DNSServiceRef; checked for in line above
        let socket = unsafe { DNSServiceRefSockFD(self.0) };
        assert!(socket != -1);

        socket
    }
    fn is_valid(&self) -> bool {
        !self.0.is_null()
    }
}
struct InternalSocket<'a> {
    fd: dnssd_sock_t,
    _lifetime: &'a (),
}
impl InternalSocket<'_> {
    fn get(&self) -> dnssd_sock_t { self.fd }
}
fn get_internal_socket<'a>(dns_service: &'a Mutex<OwnedDnsService>) -> InternalSocket<'a> {
    let guard = dns_service.lock().unwrap();
    let socket = guard.internal_socket();
    drop(guard);
    return InternalSocket { fd: socket, _lifetime: &() };
}
fn block_until_handled(dns_service: &Mutex<OwnedDnsService>) -> DNSServiceErrorType {
    let dns_service_guard = dns_service.lock().unwrap();
    let error = dns_service_guard.block_until_handled();
    drop(dns_service_guard);
    return error;
}
impl Drop for OwnedDnsService {
    fn drop(&mut self) {
        if self.is_valid() {
            // SAFETY: Must be called with a valid DNSServiceRef; checked for in line above 
            unsafe { DNSServiceRefDeallocate(self.0) };
        }
    }
}

#[cfg(not(windows))]
pub(super) mod posix {
    use super::*;
    use std::sync::Mutex;
    pub(super) fn create_pipe() -> (std::ffi::c_int, std::ffi::c_int) {
        let mut pipes = [0 as std::ffi::c_int; 2];
        // SAFETY: `pipes` contains exactly two c_int's
        let error = unsafe { pipe(&mut pipes as *mut std::ffi::c_int) };
        assert!(error == 0);
    
        (pipes[0], pipes[1])
    }
    pub(super) fn poll_indefinite(fds: &mut [libc::pollfd]) -> std::ffi::c_int {
        // -1 for indefinite timeout
        // SAFETY: fds.len() is always the correct length
        unsafe { libc::poll(fds as *mut _ as *mut libc::pollfd, fds.len() as libc::nfds_t, -1) }
    }
    
    ///
    /// Signals wait_for_status to cancel
    /// 
    pub(super) fn signal_status(pipe_writer: std::ffi::c_int) -> bool {
        if pipe_writer != -1 {
            let data = 1 as u8;
            // SAFETY: size_of will always be correct
            let error = unsafe { libc::write(pipe_writer, &data as *const _ as *const std::ffi::c_void, std::mem::size_of_val(&data)) };
            assert!(error != -1);
            return true;
        }
        return false;
    }
    
    pub(super) fn close_signal_pipe(pipe_reader: std::ffi::c_int, pipe_writer: &Mutex<std::ffi::c_int>) {
        let mut pipe_writer_guard = pipe_writer.lock().unwrap();
        // SAFETY: function is sound; just needs ffi
        unsafe {
            assert_eq!(libc::close(pipe_reader), 0);
            assert_eq!(libc::close(*pipe_writer_guard), 0);
        }
        *pipe_writer_guard = -1;
    }

    ///
    /// # Safety
    /// 
    /// `dnssd_socket` must be valid a file descriptor for the duration of the call; pipe_reader and the associated writer must not be closed during this call
    /// 
    pub(super) unsafe fn wait_for_status(dnssd_socket: std::ffi::c_int, pipe_reader: std::ffi::c_int) -> Result<(), ()> {
        let mut fds = [
            libc::pollfd {
                fd: dnssd_socket,
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: pipe_reader,
                events: libc::POLLIN,
                revents: 0,
            },
        ];
        let error = poll_indefinite(&mut fds);
        assert!(error > 0);
        
        if fds[1].revents & libc::POLLIN != 0 {
            // Event cancelled
            return Err(());
        } else if fds[0].revents & libc::POLLIN != 0 {
            return Ok(());
        } else {
            unreachable!();
        }
    }
}