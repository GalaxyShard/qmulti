mod util;
use std::{fmt::Debug, net::IpAddr, ffi::{CString, CStr}};

use async_trait::async_trait;

pub use crate::util::Protocol;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceInfo {
    name: CString,
    domain: CString,
    reg_type: CString,
    interface_index: u32,
}
impl ServiceInfo {
    pub fn name(&self) -> &CStr { self.name.as_c_str() }
    pub fn domain(&self) -> &CStr { self.domain.as_c_str() }
}
pub trait Registration {
    fn info(&self) -> &ServiceInfo;
}
#[derive(Debug)]
pub enum RegisterError {
    Offline, NotAvailable, InvalidName, Unknown, PortError(std::io::Error)
}
#[derive(Debug)]
pub enum BrowseError {
    Offline, NotAvailable, InvalidName, Unknown,
}
// #[derive(Debug)]
// pub enum BrowseReplyError {

// }
#[derive(Debug)]
pub enum ResolveError {
    Offline, Unknown,
}

#[cfg(bonjour)]
mod bonjour;

#[cfg(avahi)]
mod avahi;

#[cfg(android_nsd)]
mod android_nsd;

#[cfg(not(any(bonjour, avahi, android_nsd)))]
compile_error!("QMulti only supports Bonjour, Avahi, and Android NSD (includes Linux, Android, iOS, MacOS, and Windows)");

pub async fn publish_service(service_type: &str, protocol: Protocol, port: u16) -> Result<impl Registration, RegisterError> {
    #[cfg(bonjour)]
    return bonjour::register::RegisterFuture::new(service_type, protocol, port)?.await;
    #[cfg(avahi)]
    return avahi::register::RegisterFuture::new(service_type, protocol, port)?.await;
    #[cfg(android_nsd)]
    return compile_error!("TODO: implement Android NSD");
}
#[derive(Debug)]
pub struct ResolvedService {
    pub ip: IpAddr,
    pub port: u16,
}
#[async_trait]
pub trait FoundService: Send + Debug {
    fn info(&self) -> &ServiceInfo;
    async fn resolve(&self) -> Result<ResolvedService, ResolveError>;
}
pub trait LostService: Send + Debug {
    fn info(&self) -> &ServiceInfo;
}

#[derive(Debug)]
pub enum ServiceState {
    Found(Box<dyn FoundService>),
    Lost(Box<dyn LostService>),
    Error(BrowseError)
}
pub trait Browser {

}
pub type BrowseCallback = Box<dyn FnMut(ServiceState) + Send + 'static>;
pub fn browse_services(service_type: &str, protocol: Protocol, callback: impl FnMut(ServiceState) -> () + Send + 'static) -> Result<impl Browser, BrowseError> {
    #[cfg(bonjour)]
    return bonjour::browse::browse_services(service_type, protocol, Box::new(callback));
    #[cfg(avahi)]
    return compile_error!("TODO: implement Avahi");
    #[cfg(android_nsd)]
    return compile_error!("TODO: implement Android NSD");
}