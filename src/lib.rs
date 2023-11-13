mod util;
use std::{fmt::Debug, net::IpAddr, ffi::{CString, CStr}};

use async_trait::async_trait;

pub use crate::util::Protocol;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub trait Registration: Send {
    fn info(&self) -> &ServiceInfo;
}
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RegisterError {
    Offline, NotAvailable, InvalidName, Unknown, PortError
}
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BrowseError {
    Offline, NotAvailable, InvalidName, Unknown,
}
// #[derive(Debug)]
// pub enum BrowseReplyError {

// }
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolveError {
    Offline, Unknown,
}

#[cfg(zeroconf_impl = "bonjour")]
mod bonjour;

#[cfg(zeroconf_impl = "avahi")]
mod avahi;

#[cfg(zeroconf_impl = "android_nsd")]
mod android_nsd;

#[cfg(not(any(zeroconf_impl = "bonjour", zeroconf_impl = "avahi", zeroconf_impl = "android_nsd", zeroconf_impl = "windns")))]
compile_error!("QMulti only supports Bonjour, Avahi, Android NSD, and windns (includes Linux, Android, iOS, MacOS, and Windows)");

pub async fn publish_service(service_type: &str, protocol: Protocol, port: u16) -> Result<impl Registration, RegisterError> {
    #[cfg(zeroconf_impl = "bonjour")]
    return bonjour::register::RegisterFuture::new(service_type, protocol, port)?.await;
    #[cfg(zeroconf_impl = "avahi")]
    return avahi::register::RegisterFuture::new(service_type, protocol, port)?.await;
    #[cfg(zeroconf_impl = "android_nsd")]
    return compile_error!("TODO: implement Android NSD");
    #[cfg(zeroconf_impl = "windns")]
    return compile_error!("TODO: implement windns");
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub trait Browser: Send {

}
pub type BrowseCallback = Box<dyn FnMut(ServiceState) + Send + 'static>;
pub fn browse_services(service_type: &str, protocol: Protocol, callback: impl FnMut(ServiceState) -> () + Send + 'static) -> Result<impl Browser, BrowseError> {
    #[cfg(zeroconf_impl = "bonjour")]
    return bonjour::browse::browse_services(service_type, protocol, Box::new(callback));
    #[cfg(zeroconf_impl = "avahi")]
    return compile_error!("TODO: implement Avahi");
    #[cfg(zeroconf_impl = "android_nsd")]
    return compile_error!("TODO: implement Android NSD");
    #[cfg(zeroconf_impl = "windns")]
    return compile_error!("TODO: implement windns");
}