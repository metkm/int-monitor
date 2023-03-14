use std::net::IpAddr;

use crate::network::bandwidth::BandWidth;

#[derive(Debug, Clone, serde::Serialize)]
pub struct TableRow {
    pub state: u32,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub owning_pid: u32,
    pub bandwidth: Option<BandWidth>,
    pub create_timestamp: Option<i64>,
    pub owning_module_info: Option<[u64; 16]>,
}
