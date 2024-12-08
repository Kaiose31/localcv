use anyhow::Result;
use clap::Parser;
use opencv::prelude::*;
use opencv::{
    core::{Mat, Size, Vector},
    imgproc,
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    slice,
};
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

#[link(name = "depth", kind = "static")]
extern "C" {
    fn inference(data: *const f32) -> *mut f32;
}

pub trait Process {
    fn to_grayscale(&mut self) -> Result<()>;
    fn run_ml(&self) -> Result<Mat>;
}

impl Process for Mat {
    fn to_grayscale(&mut self) -> Result<()> {
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(self, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        *self = gray_frame;
        Ok(())
    }
    //Preprocess and run Inferenc
    fn run_ml(&self) -> Result<Mat> {
        let mut result = Mat::default();
        imgproc::resize_def(self, &mut result, Size::new(518, 518))?;

        let mut converted_image = Mat::default();
        result.convert_to(
            &mut converted_image,
            opencv::core::CV_32FC3,
            1.0 / 255.0,
            0.0,
        )?;

        let mut channels = Vector::<Mat>::new();
        opencv::core::split(&converted_image, &mut channels)?;

        let mut chw = Mat::default();
        opencv::core::vconcat(&channels, &mut chw)?;
        let flattened_data = chw.data_typed::<f32>()?;

        let depth_frame;
        unsafe {
            let depth = inference(flattened_data.as_ptr());
            if depth.is_null() {
                panic!("Null pointer");
            }

            let data_slice = slice::from_raw_parts(depth, flattened_data.len());

            depth_frame = Mat::new_rows_cols_with_data_unsafe_def(
                518,
                518,
                opencv::core::CV_32F,
                data_slice.as_ptr() as *mut _,
            )?;
        }

        let mut normalized_depth = Mat::default();
        opencv::core::normalize(
            &depth_frame,
            &mut normalized_depth,
            0.0,
            255.0,
            opencv::core::NORM_MINMAX,
            -1,
            &opencv::core::Mat::default(),
        )?;

        let mut depth_map_8u = Mat::default();
        normalized_depth.convert_to_def(&mut depth_map_8u, opencv::core::CV_8U)?;

        let mut depth_map_color = Mat::default();
        imgproc::apply_color_map(&depth_map_8u, &mut depth_map_color, imgproc::COLORMAP_JET)?;

        let mut resized = Mat::default();
        imgproc::resize_def(
            &depth_map_color,
            &mut resized,
            Size::new(self.cols(), self.rows()),
        )?;

        Ok(resized)
    }
}
