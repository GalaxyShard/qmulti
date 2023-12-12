use std::{future::Future, ptr::null_mut};

use crate::{Registration, Protocol, util::{registration_type, find_available_port}, RegisterError, ServiceInfo};
use super::bindings::*;


pub(crate) struct AvahiRegistration {
    service_info: ServiceInfo,
}
impl Registration for AvahiRegistration {
    fn info(&self) -> &ServiceInfo { &self.service_info }
}
pub(crate) struct RegisterFuture {
    
}
impl RegisterFuture {
    pub(crate) fn new(service_type: &str, protocol: Protocol, mut port: u16) -> Result<Self, RegisterError> {
        let reg_type = registration_type(service_type, protocol).map_err(|_| RegisterError::InvalidName)?;

        if port == 0 {
            port = find_available_port(protocol).map_err(|e| RegisterError::PortError(e))?;
        }

        // SAFETY: function is safe; just needs FFI
        let poll_api: *mut AvahiSimplePoll = unsafe { avahi_simple_poll_new() };
        assert!(!poll_api.is_null());


        // SAFETY: poll_api isnt null
        let raw_poll_api = unsafe { avahi_simple_poll_get(poll_api) };
        assert!(!raw_poll_api.is_null());

        let flags: AvahiClientFlags = 0;
        let callback: AvahiClientCallback = Some(handle_state_change);
        let userdata = null_mut();

        let mut error: std::ffi::c_int = 0;

        // SAFETY: should be safe; just needs ffi
        let client = unsafe { avahi_client_new(raw_poll_api, flags, callback, userdata, &mut error) };

        if client.is_null() {
            eprintln!("Avahi Error {}", error);
            return Err(RegisterError::Unknown);
        }
        
        todo!();
    }
}
impl Future for RegisterFuture {
    type Output = Result<AvahiRegistration, RegisterError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
extern "C" fn handle_state_change(client: *mut AvahiClient, state: AvahiClientState, userdata: *mut std::ffi::c_void) {
    #[allow(non_upper_case_globals)]
    match state {
        AvahiClientState_AVAHI_CLIENT_S_REGISTERING => {

        }
        AvahiClientState_AVAHI_CLIENT_S_RUNNING => {

        }
        AvahiClientState_AVAHI_CLIENT_S_COLLISION => {

        }
        AvahiClientState_AVAHI_CLIENT_FAILURE => {

        }
        AvahiClientState_AVAHI_CLIENT_CONNECTING => {
            // note: this should never run because AVAHI_CLIENT_NO_FAIL flag was not used
            panic!("Daemon not available");
        }
        _ => panic!("Invalid State")
    }
}