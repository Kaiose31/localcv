use anyhow::Result;
use clap::Parser;
use opencv::core::Mat;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::mpsc;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Number of devices
    pub devices: usize,
    ///Flag to enable/disable video stream rendering
    #[arg(long, short, action)]
    pub render: bool,
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
    pub ports: Vec<u16>,
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
            ports,
        }
    }
}

pub struct StreamData(pub usize, pub Mat, pub Mat);

#[derive(Debug, Clone)]
pub enum StreamHandler {
    Render { tx: mpsc::Sender<StreamData> },
    NoRender,
}

impl StreamHandler {
    pub fn new(render: bool, buffer_size: usize) -> (Self, Option<mpsc::Receiver<StreamData>>) {
        if render {
            let (tx, rx) = mpsc::channel::<StreamData>(buffer_size);

            (StreamHandler::Render { tx }, Some(rx))
        } else {
            (StreamHandler::NoRender, None)
        }
    }

    pub async fn send(&self, data: StreamData) -> Result<()> {
        if let Self::Render { tx } = self {
            tx.send(data).await?;
        }
        Ok(())
    }
}
