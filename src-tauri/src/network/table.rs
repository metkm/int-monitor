use std::net::{IpAddr, Ipv4Addr};

use windows::Win32::NetworkManagement::IpHelper::{MIB_TCPTABLE_OWNER_MODULE, MIB_TCPROW_OWNER_MODULE, MIB_TCPROW_OWNER_PID, TCP_TABLE_OWNER_PID_ALL, MIB_TCPTABLE_OWNER_PID};

use super::{tcp::get_tcp_buffer, udp::get_udp_buffer};

pub enum Protocol {
    Tcp,
    Udp
}

#[derive(Debug)]
pub struct Table {
    pub row_count: u32,
    pub rows: Vec<TableRow>
}

pub trait TableStructure {
    fn get_table(&self) -> Table;
}

impl TableStructure for MIB_TCPTABLE_OWNER_MODULE {
    fn get_table(&self) -> Table {
        let rows = (0..self.dwNumEntries)
            .map(|i| {
                let owner = unsafe { &*(self.table.as_ptr().add(i as usize)) };
                TableRow::from(owner)
            })
            .collect::<Vec<TableRow>>();

        Table {
            row_count: self.dwNumEntries,
            rows
        }
    }
}

impl TableStructure for MIB_TCPTABLE_OWNER_PID {
    fn get_table(&self) -> Table {
        let rows = (0..self.dwNumEntries)
            .map(|i| {
                let owner = unsafe { &*(self.table.as_ptr().add(i as usize)) };
                TableRow::from(owner)
            })
            .collect::<Vec<TableRow>>();

        Table {
            row_count: self.dwNumEntries,
            rows
        }
    }
}

#[derive(Debug)]
pub struct TableRow {
    pub state: u32,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub owning_pid: u32,
    pub create_timestamp: Option<i64>,
    pub owning_module_info: Option<[u64; 16]>
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
            owning_module_info: Some(value.OwningModuleInfo)
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
            owning_module_info: None
        }
    }
}

pub fn get_socket_info<T>(protocol: Protocol) -> Table
    where T: TableStructure
{
    let buffer = match protocol {
        Protocol::Tcp => get_tcp_buffer(TCP_TABLE_OWNER_PID_ALL),
        Protocol::Udp => get_udp_buffer()
    };

    let owner = unsafe { &*(buffer.as_ptr() as *const T) };
    let table = owner.get_table();

    table
}
