#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

include!(concat!(env!("OUT_DIR"), "/dnssd-bindings.rs"));
// use std::ffi::{c_char, c_void};

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct _DNSServiceRef_t {
//     _unused: [u8; 0],
// }
// pub type DNSServiceRef = *mut _DNSServiceRef_t;
// pub type DNSServiceErrorType = i32;
// pub type DNSServiceFlags = u32;
// pub type DNSServiceProtocol = u32;

// pub const kDNSServiceClass_IN: i32 = 1;
// pub const kDNSServiceProtocol_IPv4: u32 = 0x01;
// pub const kDNSServiceProtocol_IPv6: u32 = 0x02;
// pub const kDNSServiceProtocol_TCP: u32 = 0x20;
// pub const kDNSServiceProtocol_UDP: u32 = 0x10;

// pub const kDNSServiceFlagsAdd: u32 = 0x2;
// pub const kDNSServiceFlagsForceMulticast: u32 = 0x400;
// pub const kDNSServiceFlagsMoreComing: u32 = 0x1;
// pub const kDNSServiceFlagsNoAutoRename: u32 = 0x8;

// pub const kDNSServiceErr_NoError: i32 = 0;

// pub type DNSServiceBrowseReply = Option<unsafe extern "C" fn (
//     _sd_ref: DNSServiceRef,
//     flags: DNSServiceFlags,
//     interface_index: u32,
//     error: DNSServiceErrorType,
//     service_name: *const c_char,
//     reg_type: *const c_char,
//     reply_domain: *const c_char,
//     context: *mut c_void,
// )>;
// pub type DNSServiceRegisterReply = Option<unsafe extern "C" fn (
//     _sd_ref: DNSServiceRef,
//     flags: DNSServiceFlags,
//     error: DNSServiceErrorType,
//     name: *const c_char,
//     reg_type: *const c_char,
//     domain: *const c_char,
//     context: *mut c_void
// )>;
// pub type DNSServiceResolveReply = Option<unsafe extern "C" fn (
//     _sd_ref: DNSServiceRef,
//     flags: DNSServiceFlags,
//     interface_index: u32,
//     error: DNSServiceErrorType,
//     full_name: *const c_char,
//     host_target: *const c_char,
//     port: u16,
//     txt_len: u16,
//     txt_record: *const std::ffi::c_uchar,
//     context: *mut c_void
// )>;
// // typedef void (*DNSServiceGetAddrInfoReply)(DNSServiceRef sdRef, DNSServiceFlags flags, uint32_t interfaceIndex, DNSServiceErrorType errorCode, const char *hostname, const struct sockaddr *address, uint32_t ttl, void *context);
// pub type DNSServiceGetAddrInfoReply = Option<unsafe extern "C" fn (
//     _sd_ref: DNSServiceRef,
//     flags: DNSServiceFlags,
//     interface_index: u32,
//     error: DNSServiceErrorType,
//     host_name: *const c_char,
//     address: *const libc::sockaddr,
//     ttl: u32,
//     context: *mut c_void
// )>;


// extern "C" {
//     pub fn DNSServiceProcessResult(service_ref: DNSServiceRef) -> DNSServiceErrorType;
//     pub fn DNSServiceRefSockFD(service_ref: DNSServiceRef) -> DNSServiceErrorType;
//     pub fn DNSServiceRefDeallocate(service_ref: DNSServiceRef) -> DNSServiceErrorType;
//     pub fn DNSServiceBrowse(
//         service_ref: *mut DNSServiceRef,
//         flags: DNSServiceFlags,
//         interface_index: u32,
//         reg_type: *const c_char,
//         domain: *const c_char,
//         callback: DNSServiceBrowseReply,
//         context: *mut c_void
//     ) -> DNSServiceErrorType;
//     pub fn DNSServiceRegister(
//         service_ref: *mut DNSServiceRef,
//         flags: DNSServiceFlags,
//         interface_index: u32,
//         name: *const c_char,
//         reg_type: *const c_char,
//         domain: *const c_char,
//         host: *const c_char,
//         port: u16,
//         txt_len: u16,
//         txt_record: *const c_void,
//         callback: DNSServiceRegisterReply,
//         context: *mut c_void
//     ) -> DNSServiceErrorType;
//     pub fn DNSServiceResolve(
//         service_ref: *mut DNSServiceRef,
//         flags: DNSServiceFlags,
//         interface_index: u32,
//         name: *const c_char,
//         reg_type: *const c_char,
//         domain: *const c_char,
//         callback: DNSServiceResolveReply,
//         context: *mut c_void
//     ) -> DNSServiceErrorType;
//     pub fn DNSServiceGetAddrInfo(
//         service_ref: *mut DNSServiceRef,
//         flags: DNSServiceFlags,
//         interface_index: u32,
//         protocol: DNSServiceProtocol,
//         host_name: *const c_char,
//         callback: DNSServiceGetAddrInfoReply,
//         context: *mut c_void
//     ) -> DNSServiceErrorType;
// }