use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;

pub enum Protocol {
    Tcp,
    Udp,
    Srt,
}
pub struct ParallelServer {
    proto: Protocol,
    host: Ipv4Addr,
    ports: Vec<u16>,
    connections: Vec<SocketAddrV4>,
}

impl ParallelServer {
    pub fn new(proto: Protocol, num_devices: u8) -> Result<Self, io::Error> {
        match proto {
            Protocol::Udp | Protocol::Tcp | Protocol::Srt => todo!(),
        }
    }
}
