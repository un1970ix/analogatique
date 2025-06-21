use super::dither;
use anyhow::Result;
use image::DynamicImage;

pub fn create_thumbnail(img: &DynamicImage, apply_dither: bool) -> Result<DynamicImage> {
    let thumb = if img.width() > 400 {
        img.resize(
            400,
            400 * img.height() / img.width(),
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        img.clone()
    };

    Ok(if apply_dither {
        let gray = thumb.to_luma8();
        DynamicImage::ImageLuma8(dither::atkinson(&gray))
    } else {
        thumb
    })
}
