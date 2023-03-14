pub mod bandwidth;
pub mod models;
pub mod tcp;
pub mod udp;

use tcp::get_tcp_buffer;
use udp::get_udp_buffer;
use windows::Win32::NetworkManagement::IpHelper::TCP_TABLE_CLASS;

pub fn get_socket_info<T: models::TableStructure>(protocol: models::Protocol, class: TCP_TABLE_CLASS) -> models::Table {
    let buffer = match protocol {
        models::Protocol::Tcp => get_tcp_buffer(class),
        models::Protocol::Udp => get_udp_buffer()
    };

    let owner = unsafe { &*(buffer.as_ptr() as *const T) };
    let table = owner.get_table();

    table
}
