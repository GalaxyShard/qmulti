use std::{fmt::Display, ffi::CString, net::{TcpListener, SocketAddr, UdpSocket}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp, Udp
}
impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Protocol::Tcp => "tcp",
            Protocol::Udp => "udp",
        })
    }
}

pub(crate) fn registration_type(service_type: &str, protocol: Protocol) -> Result<CString, ()> {
    // 1-15 characters, may be letters, digits, or hyphens ()
    if !(1..=15).contains(&service_type.len())
        || service_type.bytes().any(|byte| !(byte.is_ascii_alphanumeric() || byte == b'-'))
    {
        return Err(());
    }
    let reg_type = CString::new(format!("_{}._{}", service_type, protocol)).unwrap();
    Ok(reg_type)
}
pub(crate) fn find_available_port(protocol: Protocol) -> std::io::Result<u16> {
    let socket_addr = SocketAddr::from(([127,0,0,1], 0));
    match protocol {
        Protocol::Tcp => Ok(TcpListener::bind(socket_addr)?.local_addr()?.port()),
        Protocol::Udp => Ok(UdpSocket::bind(socket_addr)?.local_addr()?.port()),
    }
}