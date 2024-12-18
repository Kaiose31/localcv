use anyhow::{Context, Result};
use clap::Parser;
use csv::Writer;
use localcv::{Args, Process, ServerConfig, StreamData, StreamHandler};
use opencv::{core::Vector, highgui, prelude::*, videoio};
use render::combine_frames;
use std::time::Instant;
use tokio::sync::mpsc;
mod render;

const BUFFER_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let servers = ServerConfig::new(args.devices, args.width, args.height);

    //Initialize Message Passing Channel
    let (tx, mut rx) = mpsc::channel::<StreamData>(BUFFER_SIZE);
    let handler = if args.render {
        StreamHandler::Render { tx }
    } else {
        StreamHandler::NoRender
    };

    //Spawn child threads
    println!("Starting Stream listeners on {:?}", servers.ports);
    let tasks: Vec<_> = servers
        .connections
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let handler = handler.clone();
            tokio::spawn(capture_stream(s.to_string(), i, handler))
        })
        .collect();

    match handler {
        StreamHandler::Render { tx: _ } => {
            println!("Starting Renderer");
            let mut frames: Vec<Option<(Mat, Mat)>> = vec![None; args.devices];
            let window_name = "video display";
            highgui::named_window_def(window_name)?;
            loop {
                if let Some(data) = rx.recv().await {
                    frames[data.0] = Some((data.1, data.2));
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
        StreamHandler::NoRender => (),
    }

    //join all listeners (synchronization)
    for task in tasks.into_iter() {
        task.await??;
    }

    Ok(())
}

//Thread function
async fn capture_stream(stream: String, index: usize, handler: StreamHandler) -> Result<()> {
    //Capture Video stream on netork socket
    let mut cap = videoio::VideoCapture::from_file_with_params(
        format!("udp://@{}?overrun_nonfatal=1&fifo_size=50000000", stream).as_str(),
        videoio::CAP_ANY,
        &Vector::<i32>::from_slice(&[
            videoio::CAP_PROP_OPEN_TIMEOUT_MSEC,
            50000,
            videoio::CAP_PROP_READ_TIMEOUT_MSEC,
            1000,
        ]),
    )?;

    println!("Started capturing stream:{}", stream);
    let mut frame = Mat::default();
    let mut writer = None;
    if let StreamHandler::NoRender = &handler {
        writer = Some(Writer::from_path(format!("outputs/{}.csv", index))?);
        if let Some(writer) = writer.as_mut() {
            writer.write_record(["frame_id", "total_latency(μs)", "algorithm_latency(μs)"])?;
        }
    }

    //process each frame
    for iter in 0.. {
        let p_latency = Instant::now();
        if !cap
            .read(&mut frame)
            .context("failed to read video frame from stream")?
        {
            eprintln!("stream {} ended", stream);
            break;
        }

        println!("stream:{} frame:{}", stream, iter);

        let start = Instant::now();
        //Run Inference
        let mut depth_frame = frame.run_ml()?;
        depth_frame.to_grayscale()?;
        frame.to_grayscale()?;
        let latency = start.elapsed();

        if let Some(writer) = writer.as_mut() {
            writer.write_record([
                iter.to_string(),
                p_latency.elapsed().as_micros().to_string(),
                latency.as_micros().to_string(),
            ])?;
        }

        handler
            .send(StreamData(index, frame.clone(), depth_frame))
            .await?
    }

    if let Some(writer) = writer.as_mut() {
        writer.flush()?;
    }
    Ok(())
}
