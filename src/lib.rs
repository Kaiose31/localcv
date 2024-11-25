use std::net::{Ipv4Addr, SocketAddrV4};

enum Protocol {
    Udp,
    Srt,
}
pub struct ServerConfig {
    // proto: Protocol,
    // host: Ipv4Addr,
    // ports: Vec<u16>,
    pub connections: Vec<SocketAddrV4>,
}

impl ServerConfig {
    pub fn new(num_devices: u8) -> Self {
        let ports: Vec<u16> = (5000..5000 + num_devices as u16).collect();
        let host = Ipv4Addr::new(0, 0, 0, 0);
        Self {
            connections: ports.iter().map(|x| SocketAddrV4::new(host, *x)).collect(),
        }
    }
}

// async fn main() -> Result<()> {

//     let (_listener, mut incoming) = SrtListener::builder().bind("0.0.0.0:5000").await?;
//     println!("Started server");
//     while let Some(request) = incoming.incoming().next().await {
//         let mut srt_socket = request.accept(None).await.unwrap();

//         tokio::spawn(async move {
//             println!("\nNew client connected: {}", srt_socket.settings().remote);
//             while let Some((_, frame_data)) = srt_socket.try_next().await.unwrap() {
//                 dbg!(frame_data);
//                 // let video_frame = decode_video_frame(&frame_data).unwrap();
//                 println!("HEREE")
//                 // dbg!(&video_frame);
//             }
//             println!("\nClient {} disconnected", srt_socket.settings().remote);
//         });
//     }
//     Ok(())
// }
