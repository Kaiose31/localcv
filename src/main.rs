use anyhow::Result;
use clap::Parser;
use csv::Writer;
use localcv::{Args, ServerConfig};
use opencv::{highgui, prelude::*, videoio};
use render::{combine_frames, ToGrayScale};
use std::time::Instant;
use tokio::sync::mpsc;
mod render;

//Performance Params
const BUFFER_SIZE: usize = 100;

// #[link(name = "depth", kind = "static")]
// extern "C" {
//     fn hello();
// }

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let servers = ServerConfig::new(args.devices, args.width, args.height);
    let (tx, mut rx) = mpsc::channel::<(usize, Mat, Mat)>(BUFFER_SIZE);

    println!("Starting Stream listeners on {:?}", servers.ports);
    let tasks: Vec<_> = servers
        .connections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let tx = tx.clone();
            tokio::spawn(capture_stream(s.to_string(), i, tx, args.render))
        })
        .collect();

    if args.render {
        println!("Starting Renderer");
        let mut frames: Vec<Option<(Mat, Mat)>> = vec![None; args.devices];
        let window_name = "video display";
        highgui::named_window_def(window_name)?;
        //start render loop
        loop {
            // This can be optimized to receive many from the buffer
            if let Some((index, frame, depth_frame)) = rx.recv().await {
                frames[index] = Some((frame, depth_frame));
            }

            let mut grid = Mat::default();
            combine_frames(&frames, &mut grid, args.height, args.width)?;
            highgui::imshow(window_name, &grid)?;
            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
    }
    //join all listeners
    for task in tasks.into_iter() {
        task.await?.expect("video capture");
    }

    Ok(())
}

async fn capture_stream(
    stream: String,
    index: usize,
    tx: mpsc::Sender<(usize, Mat, Mat)>,
    render: bool,
) -> Result<()> {
    let flags = capture_params();
    let mut writer = Writer::from_path(format!("outputs/{}.csv", index))?;
    writer.write_record(["frame_id", "decode_latency", "depth_latency"])?;

    let mut cap = videoio::VideoCapture::from_file_with_params(
        format!("udp://@{}?overrun_nonfatal=1&fifo_size=50000000", stream).as_str(),
        videoio::CAP_FFMPEG,
        &flags,
    )?;

    if !cap.is_opened()? {
        return Err(anyhow::Error::msg("Unable to open stream"));
    }

    println!("Started capturing stream: {}", stream);
    let mut frame = Mat::default();

    for iter in 0.. {
        let decode_start = Instant::now();
        if !cap.read(&mut frame)? {
            eprintln!("stream {} ended", stream);
            break;
        }
        println!("stream:{} frame:{}", stream, iter);

        let decode_latency = decode_start.elapsed();

        let depth_start = Instant::now();
        frame.convert_to_grayscale()?;
        //TODO: run ml here and send(index, (original_frame, depth_frame)) over channel
        let depth_latency = depth_start.elapsed();

        writer.write_record([
            iter.to_string(),
            decode_latency.as_micros().to_string(),
            depth_latency.as_micros().to_string(),
        ])?;

        if render {
            tx.send((index, frame.clone(), frame.clone())).await?;
        }
    }

    writer.flush()?;
    Ok(())
}

fn capture_params() -> opencv::core::Vector<i32> {
    let mut flags = opencv::core::Vector::<i32>::new();
    flags.push(videoio::CAP_PROP_OPEN_TIMEOUT_MSEC);
    flags.push(50000);
    flags.push(videoio::CAP_PROP_READ_TIMEOUT_MSEC);
    flags.push(1000);
    flags
}
