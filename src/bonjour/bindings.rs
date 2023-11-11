#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

// include!(concat!(env!("OUT_DIR"), "/dnssd-bindings.rs"));
use std::ffi::{c_char, c_void, c_uint, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DNSServiceRef_t {
    _unused: [u8; 0],
}
pub type DNSServiceRef = *mut _DNSServiceRef_t;

pub const kDNSServiceFlagsMoreComing: u32 = 1;
pub const kDNSServiceFlagsQueueRequest: u32 = 1;
pub const kDNSServiceFlagsAutoTrigger: u32 = 1;
pub const kDNSServiceFlagsAdd: u32 = 2;
pub const kDNSServiceFlagsDefault: u32 = 4;
pub const kDNSServiceFlagsNoAutoRename: u32 = 8;
pub const kDNSServiceFlagsShared: u32 = 16;
pub const kDNSServiceFlagsUnique: u32 = 32;
pub const kDNSServiceFlagsBrowseDomains: u32 = 64;
pub const kDNSServiceFlagsRegistrationDomains: u32 = 128;
pub const kDNSServiceFlagsLongLivedQuery: u32 = 256;
pub const kDNSServiceFlagsAllowRemoteQuery: u32 = 512;
pub const kDNSServiceFlagsForceMulticast: u32 = 1024;
pub const kDNSServiceFlagsForce: u32 = 2048;
pub const kDNSServiceFlagsKnownUnique: u32 = 2048;
pub const kDNSServiceFlagsReturnIntermediates: u32 = 4096;
pub const kDNSServiceFlagsShareConnection: u32 = 16384;
pub const kDNSServiceFlagsSuppressUnusable: u32 = 32768;
pub const kDNSServiceFlagsTimeout: u32 = 65536;
pub const kDNSServiceFlagsIncludeP2P: u32 = 131072;
pub const kDNSServiceFlagsWakeOnResolve: u32 = 262144;
pub const kDNSServiceFlagsBackgroundTrafficClass: u32 = 524288;
pub const kDNSServiceFlagsIncludeAWDL: u32 = 1048576;
pub const kDNSServiceFlagsEnableDNSSEC: u32 = 2097152;
pub const kDNSServiceFlagsValidate: u32 = 2097152;
pub const kDNSServiceFlagsSecure: u32 = 2097168;
pub const kDNSServiceFlagsInsecure: u32 = 2097184;
pub const kDNSServiceFlagsBogus: u32 = 2097216;
pub const kDNSServiceFlagsIndeterminate: u32 = 2097280;
pub const kDNSServiceFlagsUnicastResponse: u32 = 4194304;
pub const kDNSServiceFlagsValidateOptional: u32 = 8388608;
pub const kDNSServiceFlagsWakeOnlyService: u32 = 16777216;
pub const kDNSServiceFlagsThresholdOne: u32 = 33554432;
pub const kDNSServiceFlagsThresholdFinder: u32 = 67108864;
pub const kDNSServiceFlagsThresholdReached: u32 = 33554432;
pub const kDNSServiceFlagsPrivateOne: u32 = 8192;
pub const kDNSServiceFlagsPrivateTwo: u32 = 134217728;
pub const kDNSServiceFlagsPrivateThree: u32 = 268435456;
pub const kDNSServiceFlagsPrivateFour: u32 = 536870912;
pub const kDNSServiceFlagsPrivateFive: u32 = 1073741824;
pub const kDNSServiceFlagAnsweredFromCache: u32 = 1073741824;
pub const kDNSServiceFlagsAllowExpiredAnswers: u32 = 2147483648;
pub const kDNSServiceFlagsExpiredAnswer: u32 = 2147483648;

pub const kDNSServiceProtocol_IPv4: c_uint = 1;
pub const kDNSServiceProtocol_IPv6: c_uint = 2;
pub const kDNSServiceProtocol_UDP: c_uint = 16;
pub const kDNSServiceProtocol_TCP: c_uint = 32;

pub const kDNSServiceClass_IN: c_uint = 1;

pub const kDNSServiceType_A: c_uint = 1;
pub const kDNSServiceType_NS: c_uint = 2;
pub const kDNSServiceType_MD: c_uint = 3;
pub const kDNSServiceType_MF: c_uint = 4;
pub const kDNSServiceType_CNAME: c_uint = 5;
pub const kDNSServiceType_SOA: c_uint = 6;
pub const kDNSServiceType_MB: c_uint = 7;
pub const kDNSServiceType_MG: c_uint = 8;
pub const kDNSServiceType_MR: c_uint = 9;
pub const kDNSServiceType_NULL: c_uint = 10;
pub const kDNSServiceType_WKS: c_uint = 11;
pub const kDNSServiceType_PTR: c_uint = 12;
pub const kDNSServiceType_HINFO: c_uint = 13;
pub const kDNSServiceType_MINFO: c_uint = 14;
pub const kDNSServiceType_MX: c_uint = 15;
pub const kDNSServiceType_TXT: c_uint = 16;
pub const kDNSServiceType_RP: c_uint = 17;
pub const kDNSServiceType_AFSDB: c_uint = 18;
pub const kDNSServiceType_X25: c_uint = 19;
pub const kDNSServiceType_ISDN: c_uint = 20;
pub const kDNSServiceType_RT: c_uint = 21;
pub const kDNSServiceType_NSAP: c_uint = 22;
pub const kDNSServiceType_NSAP_PTR: c_uint = 23;
pub const kDNSServiceType_SIG: c_uint = 24;
pub const kDNSServiceType_KEY: c_uint = 25;
pub const kDNSServiceType_PX: c_uint = 26;
pub const kDNSServiceType_GPOS: c_uint = 27;
pub const kDNSServiceType_AAAA: c_uint = 28;
pub const kDNSServiceType_LOC: c_uint = 29;
pub const kDNSServiceType_NXT: c_uint = 30;
pub const kDNSServiceType_EID: c_uint = 31;
pub const kDNSServiceType_NIMLOC: c_uint = 32;
pub const kDNSServiceType_SRV: c_uint = 33;
pub const kDNSServiceType_ATMA: c_uint = 34;
pub const kDNSServiceType_NAPTR: c_uint = 35;
pub const kDNSServiceType_KX: c_uint = 36;
pub const kDNSServiceType_CERT: c_uint = 37;
pub const kDNSServiceType_A6: c_uint = 38;
pub const kDNSServiceType_DNAME: c_uint = 39;
pub const kDNSServiceType_SINK: c_uint = 40;
pub const kDNSServiceType_OPT: c_uint = 41;
pub const kDNSServiceType_APL: c_uint = 42;
pub const kDNSServiceType_DS: c_uint = 43;
pub const kDNSServiceType_SSHFP: c_uint = 44;
pub const kDNSServiceType_IPSECKEY: c_uint = 45;
pub const kDNSServiceType_RRSIG: c_uint = 46;
pub const kDNSServiceType_NSEC: c_uint = 47;
pub const kDNSServiceType_DNSKEY: c_uint = 48;
pub const kDNSServiceType_DHCID: c_uint = 49;
pub const kDNSServiceType_NSEC3: c_uint = 50;
pub const kDNSServiceType_NSEC3PARAM: c_uint = 51;
pub const kDNSServiceType_HIP: c_uint = 55;
pub const kDNSServiceType_SVCB: c_uint = 64;
pub const kDNSServiceType_HTTPS: c_uint = 65;
pub const kDNSServiceType_SPF: c_uint = 99;
pub const kDNSServiceType_UINFO: c_uint = 100;
pub const kDNSServiceType_UID: c_uint = 101;
pub const kDNSServiceType_GID: c_uint = 102;
pub const kDNSServiceType_UNSPEC: c_uint = 103;
pub const kDNSServiceType_TKEY: c_uint = 249;
pub const kDNSServiceType_TSIG: c_uint = 250;
pub const kDNSServiceType_IXFR: c_uint = 251;
pub const kDNSServiceType_AXFR: c_uint = 252;
pub const kDNSServiceType_MAILB: c_uint = 253;
pub const kDNSServiceType_MAILA: c_uint = 254;
pub const kDNSServiceType_ANY: c_uint = 255;

pub const kDNSServiceErr_NoError: c_int = 0;
pub const kDNSServiceErr_Unknown: c_int = -65537;
pub const kDNSServiceErr_NoSuchName: c_int = -65538;
pub const kDNSServiceErr_NoMemory: c_int = -65539;
pub const kDNSServiceErr_BadParam: c_int = -65540;
pub const kDNSServiceErr_BadReference: c_int = -65541;
pub const kDNSServiceErr_BadState: c_int = -65542;
pub const kDNSServiceErr_BadFlags: c_int = -65543;
pub const kDNSServiceErr_Unsupported: c_int = -65544;
pub const kDNSServiceErr_NotInitialized: c_int = -65545;
pub const kDNSServiceErr_AlreadyRegistered: c_int = -65547;
pub const kDNSServiceErr_NameConflict: c_int = -65548;
pub const kDNSServiceErr_Invalid: c_int = -65549;
pub const kDNSServiceErr_Firewall: c_int = -65550;
pub const kDNSServiceErr_Incompatible: c_int = -65551;
pub const kDNSServiceErr_BadInterfaceIndex: c_int = -65552;
pub const kDNSServiceErr_Refused: c_int = -65553;
pub const kDNSServiceErr_NoSuchRecord: c_int = -65554;
pub const kDNSServiceErr_NoAuth: c_int = -65555;
pub const kDNSServiceErr_NoSuchKey: c_int = -65556;
pub const kDNSServiceErr_NATTraversal: c_int = -65557;
pub const kDNSServiceErr_DoubleNAT: c_int = -65558;
pub const kDNSServiceErr_BadTime: c_int = -65559;
pub const kDNSServiceErr_BadSig: c_int = -65560;
pub const kDNSServiceErr_BadKey: c_int = -65561;
pub const kDNSServiceErr_Transient: c_int = -65562;
pub const kDNSServiceErr_ServiceNotRunning: c_int = -65563;
pub const kDNSServiceErr_NATPortMappingUnsupported: c_int = -65564;
pub const kDNSServiceErr_NATPortMappingDisabled: c_int = -65565;
pub const kDNSServiceErr_NoRouter: c_int = -65566;
pub const kDNSServiceErr_PollingMode: c_int = -65567;
pub const kDNSServiceErr_Timeout: c_int = -65568;
pub const kDNSServiceErr_DefunctConnection: c_int = -65569;
pub const kDNSServiceErr_PolicyDenied: c_int = -65570;
pub const kDNSServiceErr_NotPermitted: c_int = -65571;

pub type DNSServiceFlags = u32;
pub type DNSServiceProtocol = u32;
pub type DNSServiceErrorType = i32;

pub type DNSServiceBrowseReply = Option<unsafe extern "C" fn (
    _sd_ref: DNSServiceRef,
    flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    service_name: *const c_char,
    reg_type: *const c_char,
    reply_domain: *const c_char,
    context: *mut c_void,
)>;
pub type DNSServiceRegisterReply = Option<unsafe extern "C" fn (
    _sd_ref: DNSServiceRef,
    flags: DNSServiceFlags,
    error: DNSServiceErrorType,
    name: *const c_char,
    reg_type: *const c_char,
    domain: *const c_char,
    context: *mut c_void
)>;
pub type DNSServiceResolveReply = Option<unsafe extern "C" fn (
    _sd_ref: DNSServiceRef,
    flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    full_name: *const c_char,
    host_target: *const c_char,
    port: u16,
    txt_len: u16,
    txt_record: *const std::ffi::c_uchar,
    context: *mut c_void
)>;
pub type DNSServiceGetAddrInfoReply = Option<unsafe extern "C" fn (
    _sd_ref: DNSServiceRef,
    flags: DNSServiceFlags,
    interface_index: u32,
    error: DNSServiceErrorType,
    host_name: *const c_char,
    address: *const libc::sockaddr,
    ttl: u32,
    context: *mut c_void
)>;

#[cfg(windows)]
type dnssd_sock_t = libc::SOCKET;
#[cfg(not(windows))]
type dnssd_sock_t = std::ffi::c_int;

extern "C" {
    // Not available with avahi-compat-libdns_sd
    #[cfg(any(target_vendor = "apple", windows))]
    pub fn DNSServiceGetProperty(
        property: *const c_char,
        result: *mut c_void,
        size: *mut u32,
    ) -> DNSServiceErrorType;

    pub fn DNSServiceProcessResult(service_ref: DNSServiceRef) -> DNSServiceErrorType;
    pub fn DNSServiceRefSockFD(service_ref: DNSServiceRef) -> dnssd_sock_t;
    pub fn DNSServiceRefDeallocate(service_ref: DNSServiceRef) -> DNSServiceErrorType;
    pub fn DNSServiceBrowse(
        service_ref: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interface_index: u32,
        reg_type: *const c_char,
        domain: *const c_char,
        callback: DNSServiceBrowseReply,
        context: *mut c_void
    ) -> DNSServiceErrorType;
    pub fn DNSServiceRegister(
        service_ref: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interface_index: u32,
        name: *const c_char,
        reg_type: *const c_char,
        domain: *const c_char,
        host: *const c_char,
        port_be: u16,
        txt_len: u16,
        txt_record: *const c_void,
        callback: DNSServiceRegisterReply,
        context: *mut c_void
    ) -> DNSServiceErrorType;
    pub fn DNSServiceResolve(
        service_ref: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interface_index: u32,
        name: *const c_char,
        reg_type: *const c_char,
        domain: *const c_char,
        callback: DNSServiceResolveReply,
        context: *mut c_void
    ) -> DNSServiceErrorType;
    
    // Not available with avahi-compat-libdns_sd
    #[cfg(any(target_vendor = "apple", windows))]
    pub fn DNSServiceGetAddrInfo(
        service_ref: *mut DNSServiceRef,
        flags: DNSServiceFlags,
        interface_index: u32,
        protocol: DNSServiceProtocol,
        host_name: *const c_char,
        callback: DNSServiceGetAddrInfoReply,
        context: *mut c_void
    ) -> DNSServiceErrorType;
}