use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of devices
    pub devices: usize,
    /// Video stream width
    #[arg(default_value_t = 640)]
    pub width: i32,
    /// Video stream height
    #[arg(default_value_t = 480)]
    pub height: i32,
}

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
}
