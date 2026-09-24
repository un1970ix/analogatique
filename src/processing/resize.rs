use super::dither;
use image::DynamicImage;

const THUMBNAIL_WIDTH: u32 = 400;

pub fn create_thumbnail(img: &DynamicImage, apply_dither: bool) -> DynamicImage {
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

    if apply_dither {
        let gray = thumb.to_luma8();
        DynamicImage::ImageLuma8(dither::atkinson(&gray))
    } else {
        thumb
    }
}

#[cfg(test)]
mod tests {
    use super::{THUMBNAIL_WIDTH, create_thumbnail};
    use image::{DynamicImage, RgbImage};

    fn img(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgb8(RgbImage::new(w, h))
    }

    #[test]
    fn extreme_aspect_ratio_keeps_at_least_one_pixel_of_height() {
        let thumb = create_thumbnail(&img(2000, 3), false);
        assert_eq!(thumb.width(), THUMBNAIL_WIDTH);
        assert!(thumb.height() >= 1, "height collapsed to zero");
    }

    #[test]
    fn wide_images_are_scaled_to_the_thumbnail_width() {
        let thumb = create_thumbnail(&img(800, 600), false);
        assert_eq!((thumb.width(), thumb.height()), (THUMBNAIL_WIDTH, 300));
    }

    #[test]
    fn narrow_images_are_left_alone() {
        let thumb = create_thumbnail(&img(100, 80), false);
        assert_eq!((thumb.width(), thumb.height()), (100, 80));
    }
}
