use crate::prelude::*;

pub fn load_image(img_bytes: Vec<u8>, image_size: usize) -> Result<Tensor> {
    let img = image::load_from_memory(&img_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to load image: {}", e))?;
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
        .affine(2. / 255., -1.)?
        .unsqueeze(0)?;
    Ok(img)
}
