use crate::{
    codec::{TryReadFromBytes, TryWriteToBytes},
    error::NtpResult,
    ntp_message_protocol::NtpPacketHeader,
    types::{
        NtpShort, NtpTimestamp, Poll, Precision, RefId, Stratum, NTP_LEAP_NO_WARNING,
        NTP_MODE_CLIENT, NTP_VERSION_4,
    },
};
use std::{
    net::UdpSocket,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct NtpClientBuilder {
    udp_socket: UdpSocket,
    server: &'static str,
}

impl NtpClientBuilder {
    pub fn new(udp_socket: UdpSocket, server: &'static str) -> Self {
        Self { udp_socket, server }
    }

    pub fn build(self) -> NtpResult<NtpClient> {
        Ok(NtpClient {
            udp_socket: self.udp_socket,
            server: self.server,
        })
    }
}

pub struct NtpClient {
    udp_socket: UdpSocket,
    server: &'static str,
}

impl NtpClient {
    pub fn get_offset(&self) -> i64 {
        const JAN_1970: u64 = 2208988800; /* 1970 - 1900 in seconds */

        let ntp_transmit_message = NtpPacketHeader {
            leap_indicator: NTP_LEAP_NO_WARNING,
            version_number: NTP_VERSION_4,
            mode: NTP_MODE_CLIENT,
            stratum: Stratum::from(0),
            poll: Poll::from(0),
            precision: Precision::from(0),
            rootdelay: NtpShort::new(0, 0),
            rootdisp: NtpShort::new(0, 0),
            refid: RefId::from([0, 0, 0, 0]),
            reftime: NtpTimestamp::new(0, 0),
            org: NtpTimestamp::new(0, 0),
            rec: NtpTimestamp::new(0, 0),
            xmt: NtpTimestamp::new(0, 0),
        };

        let mut buffer = [0u8; 100];
        let serialized_size = ntp_transmit_message
            .try_write_to_bytes(&mut buffer)
            .unwrap();

        let send_time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        self.udp_socket
            .send_to(&buffer[..serialized_size], self.server)
            .unwrap();

        let (recv_size, _) = self.udp_socket.recv_from(&mut buffer).unwrap();
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let (packet, _) = NtpPacketHeader::try_read_from_bytes(&buffer[..recv_size]).unwrap();

        let server_transmission_time = packet.xmt;
        let server_reception_time = packet.rec;
        println!("Received NTP response {:?}", packet);

        todo!()
    }
}
