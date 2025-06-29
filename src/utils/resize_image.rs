use crate::prelude::*;
use image::{GenericImageView, ImageFormat, ImageReader};
use std::io::Cursor;

pub fn resize_image(image_data: Vec<u8>) -> Result<Vec<u8>, ApiError> {
    let img = ImageReader::new(Cursor::new(image_data))
        .with_guessed_format()
        .map_err(|_| ApiError::ImageProcessing)?
        .decode()
        .map_err(|_| ApiError::ImageProcessing)?;

    let (width, height) = img.dimensions();
    let max_dimension = 768;

    // Only resize if image is larger than target
    let resized = if width > max_dimension || height > max_dimension {
        let scale = (max_dimension as f32) / (width.max(height) as f32);
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        img.resize(new_width, new_height, image::imageops::FilterType::Triangle)
    } else {
        img
    };

    let mut buffer = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut buffer), ImageFormat::Jpeg)
        .map_err(|_| ApiError::ImageProcessing)?;

    Ok(buffer)
}
