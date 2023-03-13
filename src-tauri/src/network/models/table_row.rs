use std::net::{IpAddr, Ipv4Addr};

use windows::Win32::NetworkManagement::IpHelper::{MIB_TCPROW_OWNER_MODULE, MIB_TCPROW_OWNER_PID};

#[derive(Debug)]
pub struct TableRow {
    pub state: u32,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub owning_pid: u32,
    pub create_timestamp: Option<i64>,
    pub owning_module_info: Option<[u64; 16]>,
}

impl From<&MIB_TCPROW_OWNER_MODULE> for TableRow {
    fn from(value: &MIB_TCPROW_OWNER_MODULE) -> Self {
        Self {
            state: value.dwState,
            local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwLocalAddr))),
            local_port: u16::from_be(value.dwLocalPort as u16),
            remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwRemoteAddr))),
            remote_port: u16::from_be(value.dwRemotePort as u16),
            owning_pid: value.dwOwningPid,
            create_timestamp: Some(value.liCreateTimestamp),
            owning_module_info: Some(value.OwningModuleInfo),
        }
    }
}

impl From<&MIB_TCPROW_OWNER_PID> for TableRow {
    fn from(value: &MIB_TCPROW_OWNER_PID) -> Self {
        Self {
            state: value.dwState,
            local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwLocalAddr))),
            local_port: u16::from_be(value.dwLocalPort as u16),
            remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwRemoteAddr))),
            remote_port: u16::from_be(value.dwRemotePort as u16),
            owning_pid: value.dwOwningPid,
            create_timestamp: None,
            owning_module_info: None,
        }
    }
}
