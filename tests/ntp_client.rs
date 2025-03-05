use std::net::UdpSocket;

use demo_ntp::client::NtpClientBuilder;

#[test]
fn get_offset_from_ntp_client() {
    let udp_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let ntp_client = NtpClientBuilder::new(udp_socket, "pool.ntp.org:123")
        .build()
        .unwrap();
    let offset = ntp_client.get_offset();
    println!("Clock offset: {}", offset);
}
