use crate::prelude::*;

pub fn load_image(base64_data: String, image_size: usize) -> Result<Tensor> {
    // Decode base64 to bytes
    let img_bytes = image_base64::from_base64(base64_data);

    // Load image from bytes
    let img = image::load_from_memory(&img_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to load image: {}", e))?;

    // Resize the image
    let img = img.resize_to_fill(
        image_size as u32,
        image_size as u32,
        image::imageops::FilterType::Triangle,
    );

    let img = img.to_rgb8();
    let img = img.into_raw();
    let img = Tensor::from_vec(img, (image_size, image_size, 3), &Device::Cpu)?
        .permute((2, 0, 1))?
        .to_dtype(DType::F32)?
        .affine(2. / 255., -1.)?;
    Ok(img)
}
