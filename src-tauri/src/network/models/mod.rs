mod table;
mod table_row;

pub use table::*;
pub use table_row::*;

use windows::Win32::NetworkManagement::IpHelper::{
    MIB_TCPTABLE_OWNER_MODULE, MIB_TCPTABLE_OWNER_PID, MIB_TCPROW_LH, MIB_TCPROW_LH_0
};

impl table::TableStructure for MIB_TCPTABLE_OWNER_MODULE {
    fn get_table(&self) -> Table {
        let rows = 
            (0..self.dwNumEntries)
            .map(|index| {
                let row = unsafe { &*(self.table.as_ptr()).add(index as usize) };
                let _row_hl = self.get_row_lh(index);
                TableRow::from(row)
            })
            .collect();

        table::Table {
            row_count: self.dwNumEntries,
            rows
        }
    }

    fn get_row_lh(&self, i: u32) -> MIB_TCPROW_LH {
        let row = unsafe { &*(self.table.as_ptr()).add(i as usize) };

        MIB_TCPROW_LH {
            Anonymous: MIB_TCPROW_LH_0 { dwState: row.dwState },
            dwLocalAddr: row.dwLocalAddr,
            dwLocalPort: row.dwLocalPort,
            dwRemoteAddr: row.dwRemoteAddr,
            dwRemotePort: row.dwRemotePort
        }
    }
}

impl table::TableStructure for MIB_TCPTABLE_OWNER_PID {
    fn get_table(&self) -> Table {
        let rows = 
            (0..self.dwNumEntries)
            .map(|index| {
                let row = unsafe { &*(self.table.as_ptr()).add(index as usize) };
                let _row_hl = self.get_row_lh(index);
                TableRow::from(row)
            })
            .collect();

        table::Table {
            row_count: self.dwNumEntries,
            rows
        }
    }

    fn get_row_lh(&self, i: u32) -> MIB_TCPROW_LH {
        let row = unsafe { &*(self.table.as_ptr()).add(i as usize) };

        MIB_TCPROW_LH {
            Anonymous: MIB_TCPROW_LH_0 { dwState: row.dwState },
            dwLocalAddr: row.dwLocalAddr,
            dwLocalPort: row.dwLocalPort,
            dwRemoteAddr: row.dwRemoteAddr,
            dwRemotePort: row.dwRemotePort
        }
    }
}
