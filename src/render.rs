use anyhow::Result;
use opencv::core::{Mat, MatExprTraitConst, Size};
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
    for frame in frames.iter() {
        if let Some(frame) = frame {
            let mut resized = Mat::default();
            imgproc::resize_def(&frame, &mut resized, Size::new(width, height))?;
            frame_vec.push(frame.clone());
        } else {
            // Placeholder for empty frames
            let placeholder = Mat::ones(height, width, opencv::core::CV_8U)?.to_mat()?;
            frame_vec.push(placeholder);
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
