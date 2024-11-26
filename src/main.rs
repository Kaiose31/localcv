use anyhow::Result;
use localcv::ServerConfig;
use opencv::{highgui, prelude::*, videoio};
use render::{combine_frames, convert_to_grayscale};
use tokio::sync::mpsc;

mod render;

const HEIGHT: i32 = 480;
const WIDTH: i32 = 640;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let num_devices = args[1].parse::<usize>()?;
    let servers = ServerConfig::new(num_devices, WIDTH, HEIGHT);
    highgui::named_window_def("video display")?;

    let (tx, mut rx) = mpsc::channel::<(usize, Mat)>(1000);

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

    let mut frames: Vec<Option<Mat>> = vec![None; num_devices];

    //render
    loop {
        if let Some((index, frame)) = rx.recv().await {
            frames[index] = Some(frame);
        }

        let mut grid = Mat::default();
        combine_frames(&frames, &mut grid, HEIGHT, WIDTH)?;
        highgui::imshow("video display", &grid)?;
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

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
    let mut cap = videoio::VideoCapture::from_file_def(format!("udp://{}", stream).as_str())?;

    if !cap.is_opened().unwrap() {
        return Err(anyhow::Error::msg("Unable to open stream"));
    }

    println!("Started capturing stream: {}", stream);

    loop {
        let mut frame = Mat::default();
        cap.read(&mut frame)?;

        if !cap.read(&mut frame)? || frame.size()?.width == 0 {
            eprintln!("Failed to read frame from stream: {}", stream);
            break;
        }
        frame = convert_to_grayscale(&frame)?;

        tx.send((index, frame)).await?;
    }

    Ok(())
}
