use std::{ptr::{null, null_mut}, ffi::CStr, future::Future, task::{Poll, Waker}, sync::{Arc, Mutex}, marker::PhantomPinned, pin::Pin};

use crate::{util::{Protocol, registration_type, find_available_port}, RegisterError, Registration, ServiceInfo};
use super::{OwnedDnsService, get_internal_socket, block_until_handled};

use super::bindings::*;

pub(crate) struct BonjourRegistration {
    _dns_service: Arc<Mutex<OwnedDnsService>>,
    service_info: ServiceInfo,
}
impl Registration for BonjourRegistration {
    fn info(&self) -> &ServiceInfo { &self.service_info }
}

struct FutureState {
    waker: Option<Waker>,
    output: Option<Result<ServiceInfo, RegisterError>>,
}
struct CallbackState {
    output: Option<Result<ServiceInfo, RegisterError>>,
    _marker: PhantomPinned,
}

pub(crate) struct RegisterFuture {
    state: Arc<Mutex<FutureState>>,
    dns_service: Arc<Mutex<OwnedDnsService>>,
    #[cfg(not(windows))]
    pipe_writer: Arc<Mutex<std::ffi::c_int>>,
}

fn map_error(err: DNSServiceErrorType) -> RegisterError {
    #[allow(non_upper_case_globals)] // bug: https://github.com/rust-lang/rust/issues/39371
    match err {
        kDNSServiceErr_ServiceNotRunning => RegisterError::Offline,
        kDNSServiceErr_NoRouter => RegisterError::Offline,
        _ => RegisterError::Unknown
    }
}

impl RegisterFuture {
    pub(crate) fn new(service_type: &str, protocol: Protocol, port: u16) -> Result<Self, RegisterError> {
        #[cfg(not(windows))]
        let (reader, writer) = super::posix::create_pipe();
        #[cfg(not(windows))]
        let writer = Arc::new(Mutex::new(writer));

        let future_state = Arc::new(Mutex::new(FutureState {
            waker: None,
            output: None,
        }));
        
        let flags = 0; // note: see kDNSServiceFlagsNoAutoRename
        let name = None; // default to machine name
        let domain = None;
        let host = None; // default to machine host name
        let callback_ptr: DNSServiceRegisterReply = Some(handle_registered);

        let callback_state = Box::pin(Mutex::new(CallbackState {
            output: None,
            _marker: PhantomPinned,
        }));
        let context = &*callback_state as *const Mutex<CallbackState> as *mut std::ffi::c_void;
        
        let dns_service = Arc::new(Mutex::new(
            // SAFETY: thread::spawn > block_until_handled guarentees `callback_state` lives until after handle_registered is called
            //     handle_registered is sound under the guarentees provided by OwnedDnsService::register
            unsafe { OwnedDnsService::register(flags, name, service_type, protocol, domain, host, port, callback_ptr, context)? }
        ));

        { // hacky workaround to clone Arc between threads
            let dns_service = dns_service.clone();
            let future_state = future_state.clone();
            #[cfg(not(windows))]
            let writer = writer.clone();
            std::thread::spawn(move || register_thread(
                dns_service,
                future_state,
                callback_state,
                #[cfg(not(windows))]
                (reader, writer)
            ));
        }
        Ok(RegisterFuture {
            state: future_state,
            dns_service,
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
    /// If there are no errors returned in the callback (errorCode == 0), `name`, `regtype`, and `domain` are all valid C strings
    /// 
    unsafe fn register(
        flags: DNSServiceFlags,
        name: Option<&CStr>,
        service_type: &str,
        protocol: Protocol,
        domain: Option<&CStr>,
        host: Option<&CStr>,
        mut port: u16,
        callback_ptr: DNSServiceRegisterReply,
        context: *mut ::std::os::raw::c_void
    ) -> Result<Self, RegisterError> {

        fn str_to_ptr(str: Option<&CStr>) -> *const std::ffi::c_char {
            str.map(|v| v.as_ptr()).unwrap_or(null())
        }
        let reg_type = registration_type(service_type, protocol).map_err(|_| RegisterError::InvalidName)?;
        let mut internal_dns_ref: DNSServiceRef = null_mut();

        if port == 0 {
            port = find_available_port(protocol).map_err(|_err| RegisterError::PortError)?;
        }

        // SAFETY:
        //     sdRef must be a valid, writable memory location; &mut guarentees this
        //     regtype must be a valid pointer to a c string in the format _[A-Za-z0-9\-]{1,15}._(tcp|udp); this is validated in new_service_type
        //     callback_ptr must be safe to call (precondition)
        let error = unsafe { DNSServiceRegister(
            &mut internal_dns_ref,
            flags, 0,
            str_to_ptr(name),
            reg_type.as_ptr(),
            str_to_ptr(domain),
            str_to_ptr(host),
            port.to_be(),
            0, null(),
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

fn register_thread(
    dns_service: Arc<Mutex<OwnedDnsService>>,
    future_state: Arc<Mutex<FutureState>>,
    callback_state: Pin<Box<Mutex<CallbackState>>>,
    #[cfg(not(windows))]
    pipe: (i32, Arc<Mutex<i32>>),
) {
    // Keep these alive for the duration of the callback
    let callback_state = callback_state;
    let future_state = future_state;

    let socket = get_internal_socket(&dns_service);

    // SAFETY: dns_service lives until this thread exits; reader/writer are not closed
    #[cfg(not(windows))]
    if let Err(()) = unsafe { super::posix::wait_for_status(socket.get(), pipe.0) } {
        // future dropped; safely exit
        super::posix::close_signal_pipe(pipe.0, &pipe.1);
        return;
    } else {
        super::posix::close_signal_pipe(pipe.0, &pipe.1);
    }
    #[cfg(windows)]
    {
        todo!("implement select for windows")
        // see https://stackoverflow.com/questions/384391/how-to-signal-select-to-return-immediately
        // // SAFETY: functions are safe; just needs ffi
        // let mut fd_set = unsafe { new_fd_set() };
        // unsafe { bonjour_fd_set(dnssd_socket, &mut fd_set); }

        // let nfds = dnssd_socket + 1;

        // // SAFETY: function is safe; just needs ffi
        // let error = unsafe { select(nfds, &mut fd_set, null_mut(), null_mut(), null_mut()) };
        // assert!(error >= 0);
    }

    let error = block_until_handled(&dns_service);
    if error != 0 {
        let mut future_state_guard = future_state.lock().unwrap();
        future_state_guard.output = Some(Err(map_error(error)));
        
        if let Some(waker) = future_state_guard.waker.take() {
            drop(future_state_guard);
            waker.wake();
        }
        return;
    }

    let mut callback_state_guard = callback_state.lock().unwrap();
    if let Some(output) = callback_state_guard.output.take() {
        let mut future_state_guard = future_state.lock().unwrap();
        
        future_state_guard.output = Some(output);

        if let Some(waker) = future_state_guard.waker.take() {
            drop(future_state_guard);
            waker.wake();
        }
    } else {
        unreachable!();
    }
}

impl Future for RegisterFuture {
    type Output = Result<BonjourRegistration, RegisterError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if let Some(output) = state.output.take() {
            match output {
                Ok(info) => {
                    Poll::Ready(Ok(BonjourRegistration {
                        _dns_service: self.dns_service.clone(),
                        service_info: info,
                    }))
                }
                Err(err) => {
                    Poll::Ready(Err(err))
                }
            }
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

///
/// # Safety
/// 
/// `name`, `reg_type`, and `domain` must be valid C strings if there are no errors
/// 
/// `context` must point to a valid Mutex<CallbackState>
/// 
unsafe extern "C" fn handle_registered(
    _sd_ref: DNSServiceRef,
    _flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    name: *const ::std::os::raw::c_char,
    reg_type: *const ::std::os::raw::c_char,
    domain: *const ::std::os::raw::c_char,
    context: *mut ::std::os::raw::c_void,
) {
    // SAFETY: precondition
    let context = unsafe { &*(context as *const Mutex<CallbackState>) };
    let mut state = context.lock().unwrap();

    if error == 0 {
        assert!(!name.is_null());
        assert!(!reg_type.is_null());
        assert!(!domain.is_null());
        
        // SAFETY: error == 0; precondition
        state.output = Some(Ok(ServiceInfo {
            name: unsafe { CStr::from_ptr(name) }.to_owned(),
            domain: unsafe { CStr::from_ptr(domain) }.to_owned(),
            reg_type: unsafe { CStr::from_ptr(reg_type) }.to_owned(),
            interface_index: 0,
        }));
    } else {
        state.output = Some(Err(map_error(error)));
    }
}

#[cfg(not(windows))]
impl Drop for RegisterFuture {
    fn drop(&mut self) {
        super::posix::signal_status(*self.pipe_writer.lock().unwrap());
    }
}