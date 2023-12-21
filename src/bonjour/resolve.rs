use std::{future::Future, ffi::{CStr, CString}, ptr::null_mut, sync::{Mutex, Arc}, task::{Waker, Poll}, net::{IpAddr, Ipv4Addr, Ipv6Addr}, pin::Pin, marker::PhantomPinned};

use crate::{ServiceInfo, FoundService, ResolvedService, LostService, ResolveError};
use async_trait::async_trait;

use super::bindings::*;

use super::{BonjourLostService, BonjourFoundService, OwnedDnsService, get_internal_socket, block_until_handled};

impl LostService for BonjourLostService {
    fn info(&self) -> &ServiceInfo { &self.info }
}
struct FutureState {
    waker: Option<Waker>,
    completed: bool,
    ip: IpAddr,
    port: u16,
}
struct CallbackState {
    error: DNSServiceErrorType,
    completed: bool,
    ip: Option<IpAddr>,
    port: u16,
    full_name: Option<CString>,
    host_target: Option<CString>,
    _marker: PhantomPinned,
}

pub(crate) struct ResolveFuture {
    state: Arc<Mutex<FutureState>>,
    _dns_service: Arc<Mutex<OwnedDnsService>>,
    #[cfg(not(windows))]
    pipe_writer: Arc<Mutex<std::ffi::c_int>>,
}

fn map_error(err: DNSServiceErrorType) -> ResolveError {
    #[allow(non_upper_case_globals)] // bug: https://github.com/rust-lang/rust/issues/39371
    match err {
        kDNSServiceErr_ServiceNotRunning => ResolveError::Offline,
        kDNSServiceErr_NoRouter => ResolveError::Offline,
        _ => ResolveError::Unknown
    }
}

#[async_trait]
impl FoundService for BonjourFoundService {
    fn info(&self) -> &ServiceInfo { &self.info }

    async fn resolve(&self) -> Result<ResolvedService, ResolveError> {
        ResolveFuture::new(self)?.await
    }
}

impl ResolveFuture {
    fn new(service: &BonjourFoundService) -> Result<Self, ResolveError> {
        #[cfg(not(windows))]
        let (reader, writer) = super::posix::create_pipe();
        #[cfg(not(windows))]
        let writer = Arc::new(Mutex::new(writer));

        let future_state = Arc::new(Mutex::new(FutureState {
            completed: false,
            waker: None,
            ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 0,
        }));
        let callback_state = Box::pin(Mutex::new(CallbackState {
            error: kDNSServiceErr_NoError,
            completed: false,
            ip: None,
            port: 0,
            full_name: None,
            host_target: None,
            _marker: PhantomPinned,
        }));
        let context = &*callback_state as *const Mutex<CallbackState> as *mut std::ffi::c_void;

        let dns_service = Arc::new(Mutex::new(
            // SAFETY: thread::spawn > block_until_handled guarentees `callback_state` lives until after handle_registered is called
            //     resolve_callback is sound under the guarentees provided by OwnedDnsService::resolve
            unsafe { OwnedDnsService::resolve(
                service.info.interface_index,
                service.info.name.as_c_str(),
                service.info.reg_type.as_c_str(),
                service.info.domain.as_c_str(),
                Some(resolve_callback),
                context
            )? }
        ));

        { // hacky workaround to clone Arc between threads
            let dns_service = dns_service.clone();
            let future_state = future_state.clone();
            let interface_index = service.info.interface_index;
            #[cfg(not(windows))]
            let writer = writer.clone();
            std::thread::spawn(move || resolve_thread(
                dns_service.clone(),
                future_state.clone(),
                callback_state,
                interface_index,
                #[cfg(not(windows))]
                (reader, writer.clone()),
            ));
        }
        Ok(ResolveFuture {
            state: future_state,
            _dns_service: dns_service,
            #[cfg(not(windows))]
            pipe_writer: writer,
        })
    }
}
impl OwnedDnsService {
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the preconditions to `callback_ptr` are satisfied, given that `context` is passed directly to the callback
    /// 
    /// If there are no errors returned in the callback (errorCode == 0), `fullname` and `hosttarget` are valid C strings; port is a valid port number
    /// 
    unsafe fn resolve(
        interface_index: u32,
        name: &CStr,
        reg_type: &CStr,
        domain: &CStr,
        callback_ptr: DNSServiceResolveReply,
        context: *mut ::std::os::raw::c_void
    ) -> Result<Self, ResolveError> {
        let mut internal_dns_ref: DNSServiceRef = null_mut();

        // SAFETY: 
        //     sdRef must be a valid, writable memory location; &mut guarentees this
        //     callback_ptr must be safe to call (precondition)
        let error = unsafe { DNSServiceResolve(
            &mut internal_dns_ref,
            kDNSServiceFlagsForceMulticast | kDNSServiceFlagsIncludeP2P,
            interface_index,
            name.as_ptr(),
            reg_type.as_ptr(),
            domain.as_ptr(),
            callback_ptr,
            context
        ) };

        if error == 0 {
            Ok(OwnedDnsService(internal_dns_ref))
        } else {
            Err(map_error(error))
        }
    }
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the preconditions to `callback_ptr` are satisfied, given that `context` is passed directly to the callback
    /// 
    /// If there are no errors returned in the callback (errorCode == 0), `hostname` and `address` are valid values
    /// 
    #[cfg(target_vendor = "apple")]
    unsafe fn get_addr_info(
        interface_index: u32,
        host_name: &CStr,
        callback_ptr: DNSServiceGetAddrInfoReply,
        context: *mut ::std::os::raw::c_void
    ) -> Result<Self, ResolveError> {
        let mut internal_dns_ref: DNSServiceRef = null_mut();

        // SAFETY: 
        //     sdRef must be a valid, writable memory location; &mut guarentees this
        //     callback_ptr must be safe to call (precondition)
        let error = unsafe { DNSServiceGetAddrInfo(
            &mut internal_dns_ref,
            kDNSServiceFlagsForceMulticast,
            interface_index,
            kDNSServiceProtocol_IPv4 | kDNSServiceProtocol_IPv6,
            host_name.as_ptr(),
            callback_ptr,
            context
        ) };

        if error == 0 {
            Ok(OwnedDnsService(internal_dns_ref))
        } else {
            Err(map_error(error))
        }
    }
}
fn resolve_thread(
    resolve_service: Arc<Mutex<OwnedDnsService>>,
    future_state: Arc<Mutex<FutureState>>,
    callback_state: Pin<Box<Mutex<CallbackState>>>,
    interface_index: u32,
    #[cfg(not(windows))]
    pipe: (i32, Arc<Mutex<i32>>),
) {
    let resolve_socket = get_internal_socket(&resolve_service);

    // SAFETY: resolve_service lives until this thread exits; reader/writer are not closed
    #[cfg(not(windows))]
    if let Err(()) = unsafe { super::posix::wait_for_status(resolve_socket.get(), pipe.0) } {
        // future was dropped; safely exit
        super::posix::close_signal_pipe(pipe.0, &pipe.1);
        return;
    }
    #[cfg(windows)]
    {
        todo!("implement select for windows")
    }
    
    let error = block_until_handled(&resolve_service);
    if error != 0 {
        panic!("Unexpected error from DNSServiceProcessResult (code {})", error);
    }

    let mut callback_state_guard = callback_state.lock().unwrap();
    assert!(callback_state_guard.completed);

    if callback_state_guard.error != 0 {
        panic!("Unexpected error from callback (code {})", callback_state_guard.error);
    }
    #[allow(unused_variables)]
    let host_name = callback_state_guard.host_target.take().unwrap();
    let port = callback_state_guard.port;
    callback_state_guard.completed = false; // reset for get_addr_info
    drop(callback_state_guard);

    #[cfg(target_vendor = "apple")]
    {
        let context = &*callback_state as *const Mutex<CallbackState> as *mut std::ffi::c_void;
        
        // SAFETY:
        //     `context` lives until after this thread exits, which is after get_addr_info returns
        //     get_addr_info_callback is sound under the guarentees provided by OwnedDnsService::get_addr_info
        let addr_service = unsafe { OwnedDnsService::get_addr_info(
            interface_index,
            &host_name,
            Some(get_addr_info_callback),
            context
        ) }.unwrap(); /* TODO: proper error handling */
    
        let addr_socket = addr_service.internal_socket();

        // SAFETY: addr_service lives until this thread exits; reader/writer are not closed
        #[cfg(not(windows))]
        if let Err(()) = unsafe { super::posix::wait_for_status(addr_socket, pipe.0) } {
            super::posix::close_signal_pipe(pipe.0, &pipe.1);
            return;
        } else {
            super::posix::close_signal_pipe(pipe.0, &pipe.1);
        }
        #[cfg(windows)]
        {
            todo!("implement select for windows")
        }
        let error = addr_service.block_until_handled();
        if error != 0 {
            panic!("Unexpected error from DNSServiceProcessResult (code {})", error);
        }
    }
    
    let mut future_state_guard = future_state.lock().unwrap();
    future_state_guard.completed = true;
    future_state_guard.port = port;
    #[cfg(target_vendor = "apple")] {
        let mut callback_state_guard = callback_state.lock().unwrap();
        assert!(callback_state_guard.completed);
    
        if callback_state_guard.error != 0 {
            panic!("Unexpected error from callback (code {})", callback_state_guard.error);
        }
        future_state_guard.ip = callback_state_guard.ip.take().unwrap();
    }
    #[cfg(target_os = "linux")] { // DNSServiceGetAddrInfo not available in avahi-compat-libdns_sd
        let port_str = CString::new(port.to_string()).unwrap();
        let mut results = std::ptr::null_mut();

        // SAFETY: needed for FFI, as per Rust & libc documentation
        let mut hints: libc::addrinfo = unsafe { std::mem::zeroed() };
        hints.ai_family = libc::AF_UNSPEC;
        
        // SAFETY: FFI, function is safe
        let error = unsafe { libc::getaddrinfo(host_name.as_ptr(), port_str.as_ptr(), &hints, &mut results) };
        if error != 0 {
            panic!("getaddrinfo error (code {})", error);
        }
        assert!(!results.is_null());

        let mut iter = unsafe { results.as_ref() };
        let mut ip: Option<IpAddr> = None;
        while let Some(addrinfo) = iter {
            // println!("addrinfo found: {}", addrinfo.ai_addr);
            let sockaddr = unsafe { addrinfo.ai_addr.as_ref().unwrap() };

            if sockaddr.sa_family == (libc::AF_INET as u8).into() {
                // SAFETY: sockaddr is sockaddr_in if sa_family == AF_INET
                let octets = unsafe { &*(sockaddr as *const _ as *const libc::sockaddr_in) }.sin_addr.s_addr;
    
                ip = Some(Ipv4Addr::from(octets.to_ne_bytes()).into());
                
            } else if sockaddr.sa_family == (libc::AF_INET6 as u8).into() {
                // SAFETY: sockaddr is sockaddr_in6 if sa_family == AF_INET6
                let octets = unsafe { &*(sockaddr as *const _ as *const libc::sockaddr_in6) }.sin6_addr.s6_addr;
    
                ip = Some(Ipv6Addr::from(octets).into());
            } else {
                unreachable!();
            }

            iter = unsafe { addrinfo.ai_next.as_ref() };
        }

        // SAFETY: `results` is the output of libc::getaddrinfo
        unsafe { libc::freeaddrinfo(results) };

        
        future_state_guard.ip = ip.unwrap();
    }

    

    if let Some(waker) = future_state_guard.waker.take() {
        drop(future_state_guard);
        waker.wake();
    }
}
impl Future for ResolveFuture {
    type Output = Result<ResolvedService, ResolveError>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.completed {
            Poll::Ready(Ok(ResolvedService {
                ip: state.ip,
                port: state.port,
            }))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

///
/// # Safety
/// 
/// `full_name` and `host_target` must be valid C strings if there are no errors
/// 
/// `context` must point to a valid Mutex<CallbackState>
/// 
unsafe extern "C" fn resolve_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _interface_index: u32,
    error: DNSServiceErrorType,
    full_name: *const ::std::os::raw::c_char,
    host_target: *const ::std::os::raw::c_char,
    port_be: u16,
    _txt_len: u16,
    _txt_record: *const ::std::os::raw::c_uchar,
    context: *mut ::std::os::raw::c_void
) {
    // SAFETY: precondition
    let context = unsafe { &*(context as *const Mutex<CallbackState>) };
    let mut state = context.lock().unwrap();
    state.completed = true;

    if error == 0 {
        assert!(!full_name.is_null());
        assert!(!host_target.is_null());
        
        let port = u16::from_be(port_be);
        state.port = port;

        // SAFETY: error == 0; precondition
        state.full_name = Some(unsafe { CStr::from_ptr(full_name) }.to_owned());
        state.host_target = Some(unsafe { CStr::from_ptr(host_target) }.to_owned());
    } else {
        state.error = error;
    }
}
///
/// # Safety
/// 
/// `address` must be a valid ipv4 or ipv6 sockaddr if there are no errors
/// 
/// `context` must point to a valid Mutex<CallbackState>
/// 
#[cfg(target_vendor = "apple")]
unsafe extern "C" fn get_addr_info_callback(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    _interface_index: u32,
    error: DNSServiceErrorType,
    _hostname: *const ::std::os::raw::c_char,
    address: *const libc::sockaddr,
    _ttl: u32,
    context: *mut ::std::os::raw::c_void,
) {
    // SAFETY: precondition
    let context = unsafe { &*(context as *const Mutex<CallbackState>) };
    let mut state = context.lock().unwrap();
    state.completed = true;

    if error == 0 {
        // SAFETY: precondition
        let address = unsafe { &*(address as *const libc::sockaddr) };

        if address.sa_family == (libc::AF_INET as u8).into() {
            // SAFETY: sockaddr is sockaddr_in if sa_family == AF_INET
            let octets = unsafe { &*(address as *const _ as *const libc::sockaddr_in) }.sin_addr.s_addr;

            state.ip = Some(Ipv4Addr::from(octets.to_ne_bytes()).into());
            
        } else if address.sa_family == (libc::AF_INET6 as u8).into() {
            // SAFETY: sockaddr is sockaddr_in6 if sa_family == AF_INET6
            let octets = unsafe { &*(address as *const _ as *const libc::sockaddr_in6) }.sin6_addr.s6_addr;

            state.ip = Some(Ipv6Addr::from(octets).into());
        } else {
            unreachable!();
        }
    } else {
        state.error = error;
    }
}

#[cfg(not(windows))]
impl Drop for ResolveFuture {
    fn drop(&mut self) {
        super::posix::signal_status(*self.pipe_writer.lock().unwrap());
    }
}