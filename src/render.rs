use anyhow::Result;
use opencv::core::{Mat, MatExprTraitConst, Vector};
use opencv::imgproc;

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

pub fn convert_to_grayscale(input_frame: &Mat) -> Result<Mat> {
    let mut gray_frame = Mat::default();
    imgproc::cvt_color(input_frame, &mut gray_frame, imgproc::COLOR_BGR2GRAY, 0)?;
    Ok(gray_frame)
}
