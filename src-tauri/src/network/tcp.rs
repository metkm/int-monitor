use std::ffi::c_void;

use windows::Win32::{
    NetworkManagement::IpHelper::{
        GetExtendedTcpTable, TCP_TABLE_CLASS,
    },
    Networking::WinSock::AF_INET,
};

pub fn get_tcp_buffer(table_class: TCP_TABLE_CLASS) -> Vec<u8> {
    let mut dw_size = 0;
    let mut err_code = unsafe {
        GetExtendedTcpTable(
            None,
            &mut dw_size,
            false,
            AF_INET.0 as u32,
            table_class,
            0,
        )
    };

    let mut table: Vec<u8> = Vec::new();
    while err_code != 0 {
        table = Vec::with_capacity(dw_size as usize);
        err_code = unsafe {
            GetExtendedTcpTable(
                Some(table.as_ptr() as *mut c_void),
                &mut dw_size,
                false,
                AF_INET.0 as u32,
                table_class,
                0,
            )
        }
    }
    table
}
