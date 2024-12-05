use anyhow::Result;
use opencv::core::{Mat, MatExprTraitConst, MatTraitConst, MatTraitConstManual, Size, Vector};
use opencv::imgproc;
use std::slice;

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

pub fn combine_frames(
    frames: &[Option<(Mat, Mat)>],
    output: &mut Mat,
    height: i32,
    width: i32,
) -> Result<()> {
    let mut frame_vec = Vector::<Mat>::new();
    let placeholder = Mat::zeros(height, width, opencv::core::CV_8U)?.to_mat()?;
    let mut src = Vector::<Mat>::with_capacity(2);
    let mut placeholder_vec = Vector::<Mat>::with_capacity(2);
    placeholder_vec.push(placeholder.clone());
    placeholder_vec.push(placeholder.clone());
    let mut vplaceholder = Mat::default();
    opencv::core::vconcat(&placeholder_vec, &mut vplaceholder)?;

    for frame in frames.iter() {
        if let Some((frame, depth_frame)) = frame {
            let mut vertical = Mat::default();
            src.push(frame.clone());
            src.push(depth_frame.clone());
            opencv::core::vconcat(&src, &mut vertical)?;
            src.clear();
            frame_vec.push(vertical.clone());
        } else {
            // Placeholder for empty frames
            frame_vec.push(vplaceholder.clone());
        }
    }

    opencv::core::hconcat(&frame_vec, output)?;
    Ok(())
}
