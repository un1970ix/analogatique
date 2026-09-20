use super::dither;
use anyhow::Result;
use image::DynamicImage;

const THUMBNAIL_WIDTH: u32 = 400;

pub fn create_thumbnail(img: &DynamicImage, apply_dither: bool) -> Result<DynamicImage> {
    let thumb = if img.width() > THUMBNAIL_WIDTH {
        let height =
            u64::from(THUMBNAIL_WIDTH) * u64::from(img.height()) / u64::from(img.width().max(1));

        img.resize(
            THUMBNAIL_WIDTH,
            u32::try_from(height).unwrap_or(u32::MAX).max(1),
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
