use anyhow::Result;
use opencv::core::{Mat, MatExprTraitConst};
use opencv::imgproc;
//Horizontally combine frame for display (idx, frame), (idx + 1, frame) ..
//TODO: vertically combine (idx, frame) with (idx,depth_frame)
pub fn combine_frames(
    frames: &[Option<Mat>],
    output: &mut Mat,
    height: i32,
    width: i32,
) -> Result<()> {
    let mut frame_vec = opencv::core::Vector::<Mat>::new();
    let placeholder = Mat::ones(height, width, opencv::core::CV_8U)?.to_mat()?;

    for frame in frames.iter() {
        if let Some(frame) = frame {
            frame_vec.push(frame.clone());
        } else {
            // Placeholder for empty frames
            frame_vec.push(placeholder.clone());
        }
    }

    opencv::core::hconcat(&frame_vec, output)?;
    Ok(())
}

pub fn convert_to_grayscale(input_frame: &Mat) -> Result<Mat> {
    let mut gray_frame = Mat::default();
    imgproc::cvt_color(input_frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray_frame)
}
