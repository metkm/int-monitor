// use windows::Win32::NetworkManagement::IpHelper::{
//     MIB_TCPROW_LH, MIB_TCPROW_LH_0, MIB_TCPROW_OWNER_MODULE, MIB_TCPROW_OWNER_PID,
//     MIB_TCPTABLE_OWNER_MODULE, MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
// };

// use super::{
//     bandwidth::{get_connection_stats},
//     tcp::get_tcp_buffer,
//     udp::get_udp_buffer,
// };

// pub enum Protocol {
//     Tcp,
//     Udp,
// }

// #[derive(Debug)]
// pub struct Table {
//     pub row_count: u32,
//     pub rows: Vec<TableRow>,
// }

// pub trait TableStructure {
//     fn get_table(&self) -> Table;
// }

// impl TableStructure for MIB_TCPTABLE_OWNER_MODULE {
//     fn get_table(&self) -> Table {
//         let rows = (0..self.dwNumEntries)
//             .map(|i| {
//                 let row = unsafe { &*(self.table.as_ptr().add(i as usize)) };

// let x = MIB_TCPROW_LH {
//     Anonymous: MIB_TCPROW_LH_0 {
//         dwState: row.dwState,
//     },
//     dwLocalAddr: row.dwLocalAddr,
//     dwLocalPort: row.dwLocalPort,
//     dwRemoteAddr: row.dwRemoteAddr,
//     dwRemotePort: row.dwRemotePort,
// };
//                 get_connection_stats(&x);

//                 TableRow::from(row)
//             })
//             .collect::<Vec<TableRow>>();

//         Table {
//             row_count: self.dwNumEntries,
//             rows,
//         }
//     }
// }

// impl TableStructure for MIB_TCPTABLE_OWNER_PID {
//     fn get_table(&self) -> Table {
//         let rows = (0..self.dwNumEntries)
//             .map(|i| {
//                 let row = unsafe { &*(self.table.as_ptr().add(i as usize)) };

// let x = MIB_TCPROW_LH {
//     Anonymous: MIB_TCPROW_LH_0 {
//         dwState: row.dwState,
//     },
//     dwLocalAddr: row.dwLocalAddr,
//     dwLocalPort: row.dwLocalPort,
//     dwRemoteAddr: row.dwRemoteAddr,
//     dwRemotePort: row.dwRemotePort,
// };

//                 if let Some(stats) = get_connection_stats(&x) {
//                     if row.dwOwningPid == 8248 {
//                         println!(
//                             "{:?} - local addr {:?} remote addr {:?} pid: {:?}",
//                             stats.inbound / 1000,
//                             row.dwLocalAddr,
//                             row.dwRemoteAddr,
//                             row.dwOwningPid
//                         )
//                     }
//                 }

//                 TableRow::from(row)
//             })
//             .collect::<Vec<TableRow>>();

//         Table {
//             row_count: self.dwNumEntries,
//             rows,
//         }
//     }
// }

// pub enum Protocol {
//     Tcp,
//     Udp
// }

// // pub trait TableStructure {
// //     fn get_table(&self) -> Table;
// // }

// pub fn get_socket_info<T>(protocol: Protocol) -> Table
// where
//     T: TableStructure,
// {
//     // let buffer = match protocol {
//     //     Protocol::Tcp => get_tcp_buffer(TCP_TABLE_OWNER_PID_ALL),
//     //     Protocol::Udp => get_udp_buffer(),
//     // };

//     // let owner = unsafe { &*(buffer.as_ptr() as *const T) };
//     // let table = owner.get_table();

//     // table
// }
