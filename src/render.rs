use anyhow::Result;
use opencv::core::{Mat, MatExprTraitConst, MatTraitConst, Vector};
use opencv::imgproc;

#[link(name = "depth", kind = "static")]
extern "C" {
    fn test_inference(data: *const f32, size: i32);
}
pub trait Process {
    fn convert_to_grayscale(&mut self) -> Result<()>;
    fn inference(&mut self) -> Result<()>;
}

impl Process for Mat {
    fn convert_to_grayscale(&mut self) -> Result<()> {
        let mut gray_frame = Mat::default();
        imgproc::cvt_color(self, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
        *self = gray_frame;
        Ok(())
    }

    fn inference(&mut self) -> Result<()> {
        Ok(())
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
