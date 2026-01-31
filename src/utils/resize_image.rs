use crate::prelude::*;
use image::{codecs::jpeg::JpegEncoder, ImageReader};
use std::io::Cursor;

pub fn normalize_image(image_data: Vec<u8>) -> Result<Vec<u8>, ApiError> {
    let img = ImageReader::new(Cursor::new(&image_data))
        .with_guessed_format()
        .map_err(|_| ApiError::ImageProcessing)?
        .decode()
        .map_err(|_| ApiError::ImageProcessing)?;

    let (width, height) = (img.width(), img.height());
    let max_dim = 2000;

    let img = if width > max_dim || height > max_dim {
        img.thumbnail(max_dim, max_dim)
    } else {
        img
    };

    let (width, height) = (img.width(), img.height());
    let mut buffer = Vec::new();

    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 90);

    let rgb = img.to_rgb8();

    encoder
        .encode(rgb.as_raw(), width, height, image::ExtendedColorType::Rgb8)
        .map_err(|_| ApiError::ImageProcessing)?;

    Ok(buffer)
}
