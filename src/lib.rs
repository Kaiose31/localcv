use std::net::{Ipv4Addr, SocketAddrV4};
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub width: i32,
    pub height: i32,
    pub connections: Vec<SocketAddrV4>,
}

impl ServerConfig {
    pub fn new(num_devices: usize, width: i32, height: i32) -> Self {
        let ports: Vec<u16> = (5000..5000 + num_devices as u16).collect();
        let host = Ipv4Addr::new(0, 0, 0, 0);
        Self {
            connections: ports.iter().map(|x| SocketAddrV4::new(host, *x)).collect(),
            width,
            height,
        }
    }

    pub fn get_num_devices(&self) -> usize {
        self.connections.len()
    }
}
