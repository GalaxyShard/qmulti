use std::{ffi::CStr, ptr::{null, null_mut}, sync::{Arc, Mutex}};

use crate::{Browser, BrowseError, BrowseCallback, ServiceState, ServiceInfo};
use crate::util::{Protocol, registration_type};
use super::{OwnedDnsService, BonjourFoundService, BonjourLostService, get_internal_socket, block_until_handled};

use crate::bindings::*;

pub(crate) struct BonjourBrowser {
    _dns_service: Arc<Mutex<OwnedDnsService>>,
    #[cfg(not(windows))]
    pipe_writer: Arc<Mutex<std::ffi::c_int>>,
}
impl Browser for BonjourBrowser {
    
}
pub(crate) fn browse_services(service_type: &str, protocol: Protocol, callback: BrowseCallback) -> Result<BonjourBrowser, BrowseError> {
    #[cfg(not(windows))]
    let (reader, writer) = super::posix::create_pipe();
    #[cfg(not(windows))]
    let writer = Arc::new(Mutex::new(writer));

    let domain = None;
    let internal_callback: DNSServiceBrowseReply = Some(browse_callback);

    let context = Box::new(Mutex::new(callback));
    let raw_context = &*context as *const Mutex<BrowseCallback> as *mut std::ffi::c_void;
    
    let dns_service = Arc::new(Mutex::new(
        // SAFETY: thread::spawn keeps `context` alive until after browsing stops
        //     browse_callback is sound under the guarentees provided by OwnedDnsService::browse
        unsafe { OwnedDnsService::browse(service_type, protocol, domain, internal_callback, raw_context)? }
    ));

    { // hacky workaround to clone Arc between threads
        let dns_service = dns_service.clone();
        #[cfg(not(windows))]
        let writer = writer.clone();
        std::thread::spawn(move || browse_thread(
            dns_service,
            context,
            #[cfg(not(windows))]
            (reader, writer)
        ));
    }

    Ok(BonjourBrowser {
        _dns_service: dns_service,
        #[cfg(not(windows))]
        pipe_writer: writer,
    })
}
impl OwnedDnsService {
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the preconditions to `callback_ptr` are satisfied, given that `context` is passed directly to the callback
    /// 
    /// If there are no errors returned in the callback (errorCode == 0), `serviceName`, `regtype`, and `replyDomain` are all valid C strings
    /// 
    unsafe fn browse(
        service_type: &str,
        protocol: Protocol,
        domain: Option<&CStr>,
        callback_ptr: DNSServiceBrowseReply,
        context: *mut ::std::os::raw::c_void
    ) -> Result<Self, BrowseError> {

        let reg_type = registration_type(service_type, protocol).map_err(|_| BrowseError::InvalidName)?;
        let mut internal_dns_ref: DNSServiceRef = null_mut();

        // SAFETY:
        //     sdRef must be a valid, writable memory location; &mut guarentees this
        //     regtype must be a valid pointer to a c string in the format _[A-Za-z0-9\-]{1,15}._(tcp|udp); this is validated in new_service_type
        //     callback_ptr must be safe to call (precondition)
        let error = unsafe { DNSServiceBrowse(
            &mut internal_dns_ref, 0, 0,
            reg_type.as_ptr(),
            domain.map(|v| v.as_ptr()).unwrap_or(null()),
            callback_ptr,
            context
        ) };
        if error == 0 {
            Ok(OwnedDnsService(internal_dns_ref))
        } else {
            Err(BrowseError::Unknown)
        }
    }
}

fn browse_thread(
    dns_service: Arc<Mutex<OwnedDnsService>>,
    _context: Box<Mutex<BrowseCallback>>,
    #[cfg(not(windows))]
    pipe: (i32, Arc<Mutex<i32>>),
) {
    loop {
        let socket = get_internal_socket(&dns_service);

        // SAFETY: dns_service lives until this thread exits; reader/writer are not closed
        #[cfg(not(windows))]
        if let Err(()) = unsafe { super::posix::wait_for_status(socket.get(), pipe.0) } {
            super::posix::close_signal_pipe(pipe.0, &pipe.1);
            return;
        }
        #[cfg(windows)]
        {
            todo!("implement select for windows")
        }

        let error = block_until_handled(&dns_service);
        if error != 0 {
            panic!("Unexpected error from DNSServiceProcessResult (code {})", error);
        }
    }
}

///
/// # Safety
/// 
/// `service_name`, `reg_type`, and `reply_domain` must be valid C strings if there are no errors
/// 
/// `context` must point to a valid Mutex<BrowseCallback>
/// 
unsafe extern "C" fn browse_callback(
    _sd_ref: DNSServiceRef,
    flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    service_name: *const ::std::os::raw::c_char,
    reg_type: *const ::std::os::raw::c_char,
    reply_domain: *const ::std::os::raw::c_char,
    context: *mut ::std::os::raw::c_void,
) {
    // SAFETY: precondition
    let callback = unsafe { &*(context as *const Mutex<BrowseCallback>) };
    let mut callback = callback.lock().unwrap();
    if error != 0 {
        eprintln!("Unexpected error: {error}");
        callback(ServiceState::Error(BrowseError::Unknown));
        return;
    }

    // SAFETY: error == 0; precondition
    let info = ServiceInfo {
        name: unsafe { CStr::from_ptr(service_name) }.to_owned(),
        domain: unsafe { CStr::from_ptr(reply_domain) }.to_owned(),
        reg_type: unsafe { CStr::from_ptr(reg_type) }.to_owned(),
        interface_index,
    };

    let _results_are_queued = flags & kDNSServiceFlagsMoreComing != 0;
    
    if flags & kDNSServiceFlagsAdd != 0 {
        callback(ServiceState::Found(Box::new(BonjourFoundService { info })))
    } else {
        callback(ServiceState::Lost(Box::new(BonjourLostService { info })))
    }
}

#[cfg(not(windows))]
impl Drop for BonjourBrowser {
    fn drop(&mut self) {
        super::posix::signal_status(*self.pipe_writer.lock().unwrap());
    }
}