use anyhow::Result;
use localcv::ServerConfig;
use opencv::core::CV_8U;
use opencv::{highgui, prelude::*, videoio};

use tokio::sync::mpsc;

const NUM_DEVICES: usize = 1;

#[tokio::main]
async fn main() -> Result<()> {
    let servers = ServerConfig::new(NUM_DEVICES as u8);
    highgui::named_window("video capture", highgui::WINDOW_AUTOSIZE)?;
    let (tx, mut rx) = mpsc::channel::<(usize, Mat)>(100);

    // capture_stream(servers.connections[0].to_string()).await?;

    let tasks: Vec<_> = servers
        .connections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let tx = tx.clone();
            tokio::spawn(capture_stream(s.to_string(), i, tx))
        })
        .collect();

    let mut frames: Vec<Option<Mat>> = vec![None; NUM_DEVICES];
    loop {
        if let Some((index, frame)) = rx.recv().await {
            frames[index] = Some(frame);
        }

        // Combine frames into a grid
        let mut grid = Mat::default();
        combine_frames(&frames, &mut grid)?;
        highgui::imshow("video capture", &grid)?;
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    for task in tasks {
        task.await?.expect("capturing video")
    }

    Ok(())
}

async fn capture_stream(
    stream: String,
    index: usize,
    tx: mpsc::Sender<(usize, Mat)>,
) -> Result<()> {
    let mut cap = videoio::VideoCapture::from_file(
        format!("udp://{}", stream).as_str(),
        videoio::CAP_FFMPEG,
    )?;

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

        tx.send((index, frame)).await.unwrap();
    }

    Ok(())
}

fn combine_frames(frames: &[Option<Mat>], output: &mut Mat) -> Result<()> {
    let mut frame_vec = opencv::core::Vector::<Mat>::new();

    for frame in frames.iter() {
        if let Some(frame) = frame {
            frame_vec.push(frame.clone());
        } else {
            // Placeholder for empty frames
            let placeholder = Mat::ones(480, 640, CV_8U)?.to_mat()?;
            frame_vec.push(placeholder);
        }
    }

    opencv::core::hconcat(&frame_vec, output)?;

    Ok(())
}
