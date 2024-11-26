use anyhow::Result;
use clap::Parser;
use localcv::{Args, ServerConfig};
use opencv::{highgui, prelude::*, videoio};
use render::{combine_frames, convert_to_grayscale};
use tokio::sync::mpsc;

mod render;

// #[link(name = "depth", kind = "static")]
// extern "C" {
//     fn hello();
// }

#[tokio::main]
async fn main() -> Result<()> {
    let window_name = "video display";
    let args = Args::parse();
    let servers = ServerConfig::new(args.devices, args.width, args.height);
    highgui::named_window_def(window_name)?;
    let (tx, mut rx) = mpsc::channel::<(usize, Mat)>(4096);

    // start listeners
    let tasks: Vec<_> = servers
        .connections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let tx = tx.clone();
            tokio::spawn(capture_stream(s.to_string(), i, tx))
        })
        .collect();

    //TODO: record depth frames additionally.
    let mut frames: Vec<Option<Mat>> = vec![None; args.devices];

    //start render loop
    loop {
        // This can be optimized to receive many from the buffer
        if let Some((index, frame)) = rx.recv().await {
            frames[index] = Some(frame);
        }

        let mut grid = Mat::default();
        combine_frames(&frames, &mut grid, args.height, args.width)?;
        highgui::imshow(window_name, &grid)?;
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
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
    tx: mpsc::Sender<(usize, Mat)>,
) -> Result<()> {
    let flags = capture_params();

    let mut cap = videoio::VideoCapture::from_file_with_params(
        format!("udp://{}", stream).as_str(),
        videoio::CAP_FFMPEG,
        &flags,
    )?;
    cap.set(videoio::CAP_PROP_BUFFERSIZE, 4096.0)?;

    if !cap.is_opened().unwrap() {
        return Err(anyhow::Error::msg("Unable to open stream"));
    }

    println!("Started capturing stream: {}", stream);
    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;
        frame = convert_to_grayscale(&frame)?;
        if frame.empty() {
            eprintln!("No frame received");
            break;
        }
        //TODO: run ml here and send(index, (original_frame, depth_frame)) over channel
        tx.send((index, frame)).await?;
    }

    Ok(())
}

fn capture_params() -> opencv::core::Vector<i32> {
    let mut flags = opencv::core::Vector::<i32>::new();
    flags.push(videoio::CAP_PROP_OPEN_TIMEOUT_MSEC);
    flags.push(50000);
    flags.push(videoio::CAP_PROP_READ_TIMEOUT_MSEC);
    flags.push(50000);
    flags
}
