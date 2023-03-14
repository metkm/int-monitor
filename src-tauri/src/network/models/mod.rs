mod table;
mod table_row;

use std::net::{IpAddr, Ipv4Addr};

pub use table::*;
pub use table_row::*;

use windows::Win32::NetworkManagement::IpHelper::{
    MIB_TCPROW_LH, MIB_TCPROW_LH_0, MIB_TCPTABLE_OWNER_MODULE, MIB_TCPTABLE_OWNER_PID,
};

use super::bandwidth::get_connection_stats;

impl table::TableStructure for MIB_TCPTABLE_OWNER_MODULE {
    fn get_table(&self) -> Table {
        let rows = (0..self.dwNumEntries)
            .map(|index| {
                let value = unsafe { &*(self.table.as_ptr()).add(index as usize) };
                let row_hl = self.get_row_lh(index);
                let bandwidth = get_connection_stats(&row_hl);

                TableRow {
                    state: value.dwState,
                    local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwLocalAddr))),
                    local_port: u16::from_be(value.dwLocalPort as u16),
                    remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwRemoteAddr))),
                    remote_port: u16::from_be(value.dwRemotePort as u16),
                    owning_pid: value.dwOwningPid,
                    bandwidth,
                    create_timestamp: Some(value.liCreateTimestamp),
                    owning_module_info: Some(value.OwningModuleInfo),
                }
            })
            .collect();

        table::Table {
            row_count: self.dwNumEntries,
            rows,
        }
    }

    fn get_row_lh(&self, i: u32) -> MIB_TCPROW_LH {
        let row = unsafe { &*(self.table.as_ptr()).add(i as usize) };

        MIB_TCPROW_LH {
            Anonymous: MIB_TCPROW_LH_0 {
                dwState: row.dwState,
            },
            dwLocalAddr: row.dwLocalAddr,
            dwLocalPort: row.dwLocalPort,
            dwRemoteAddr: row.dwRemoteAddr,
            dwRemotePort: row.dwRemotePort,
        }
    }
}

impl table::TableStructure for MIB_TCPTABLE_OWNER_PID {
    fn get_table(&self) -> Table {
        let rows = (0..self.dwNumEntries)
            .map(|index| {
                let value = unsafe { &*(self.table.as_ptr()).add(index as usize) };
                let row_hl = self.get_row_lh(index);
                let bandwidth = get_connection_stats(&row_hl);

                TableRow {
                    state: value.dwState,
                    local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwLocalAddr))),
                    local_port: u16::from_be(value.dwLocalPort as u16),
                    remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(value.dwRemoteAddr))),
                    remote_port: u16::from_be(value.dwRemotePort as u16),
                    owning_pid: value.dwOwningPid,
                    bandwidth,
                    create_timestamp: None,
                    owning_module_info: None,
                }
            })
            .collect();

        table::Table {
            row_count: self.dwNumEntries,
            rows,
        }
    }

    fn get_row_lh(&self, i: u32) -> MIB_TCPROW_LH {
        let row = unsafe { &*(self.table.as_ptr()).add(i as usize) };

        MIB_TCPROW_LH {
            Anonymous: MIB_TCPROW_LH_0 {
                dwState: row.dwState,
            },
            dwLocalAddr: row.dwLocalAddr,
            dwLocalPort: row.dwLocalPort,
            dwRemoteAddr: row.dwRemoteAddr,
            dwRemotePort: row.dwRemotePort,
        }
    }
}
