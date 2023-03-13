use windows::Win32::NetworkManagement::IpHelper::MIB_TCPROW_LH;

use super::TableRow;

pub trait TableStructure {
    fn get_table(&self) -> Table;
    fn get_row_lh(&self, i: u32) -> MIB_TCPROW_LH;
}

#[derive(Debug)]
pub enum Protocol {
    Tcp,
    Udp
}

#[derive(Debug)]
pub struct Table {
    pub row_count: u32,
    pub rows: Vec<TableRow>
}
