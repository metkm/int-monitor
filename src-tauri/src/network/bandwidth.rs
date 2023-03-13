use windows::Win32::NetworkManagement::IpHelper::{
    GetPerTcpConnectionEStats, SetPerTcpConnectionEStats, TCP_ESTATS_BANDWIDTH_ROD_v0,
    TCP_ESTATS_BANDWIDTH_RW_v0, TcpBoolOptEnabled, TcpConnectionEstatsBandwidth, MIB_TCPROW_LH,
};

#[derive(Debug, serde::Serialize)]
pub struct BandWidth {
    pub outbound: u64,
    pub inbound: u64,
}

impl From<&TCP_ESTATS_BANDWIDTH_ROD_v0> for BandWidth {
    fn from(value: &TCP_ESTATS_BANDWIDTH_ROD_v0) -> Self {
        Self {
            outbound: value.OutboundBandwidth,
            inbound: value.InboundBandwidth,
        }
    }
}

pub fn get_connection_stats(row: &MIB_TCPROW_LH) -> Option<BandWidth> {
    let bandwidth_rw = TCP_ESTATS_BANDWIDTH_RW_v0 {
        EnableCollectionInbound: TcpBoolOptEnabled,
        EnableCollectionOutbound: TcpBoolOptEnabled,
    };

    let mut err_code = unsafe {
        let rw_buffer = std::slice::from_raw_parts(
            &bandwidth_rw as *const TCP_ESTATS_BANDWIDTH_RW_v0 as *const u8,
            std::mem::size_of::<TCP_ESTATS_BANDWIDTH_RW_v0>(),
        );

        SetPerTcpConnectionEStats(
            row as *const MIB_TCPROW_LH,
            TcpConnectionEstatsBandwidth,
            rw_buffer,
            0,
            0,
        )
    };

    if err_code != 0 {
        return None;
    }
    err_code = 1;

    let mut iteration = 0;
    let mut rw_buffer = [0; std::mem::size_of::<TCP_ESTATS_BANDWIDTH_RW_v0>()];
    let mut rod_buffer = [0; std::mem::size_of::<TCP_ESTATS_BANDWIDTH_ROD_v0>()];

    while err_code != 0 {
        err_code = unsafe {
            GetPerTcpConnectionEStats(
                row as *const MIB_TCPROW_LH,
                TcpConnectionEstatsBandwidth,
                Some(&mut rw_buffer),
                0,
                None,
                0,
                Some(&mut rod_buffer),
                0,
            )
        };

        iteration += 1;
        if iteration > 10 {
            return None;
        }
    }

    let stats = unsafe { &*(rod_buffer.as_ptr() as *const TCP_ESTATS_BANDWIDTH_ROD_v0) };
    Some(BandWidth::from(stats))
}
