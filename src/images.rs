use crate::dither;
use crate::metadata::PhotoMetadata;
use anyhow::{Context, Result};
use exif::{In, Reader, Tag};
use image::{DynamicImage, ImageFormat};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Photo {
    pub original: String,
    pub thumb_filename: String,
    pub full_filename: String,
    pub meta: PhotoMetadata,
    pub width: u32,
    pub height: u32,
}

pub fn process_all(metadata: &HashMap<String, PhotoMetadata>, dither: bool) -> Result<Vec<Photo>> {
    create_dirs()?;

    let mut image_paths = Vec::new();
    let photos_dir = Path::new("photos");

    if !photos_dir.exists() {
        return Err(anyhow::anyhow!(
            "Photos directory not found. Please create a 'photos' directory with your images."
        ));
    }

    for entry in fs::read_dir(photos_dir)? {
        let path = entry?.path();

        if is_image(&path) {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(meta) = metadata.get(name) {
                    image_paths.push((path, meta.clone()));
                }
            }
        }
    }

    let mut photos: Vec<Photo> = image_paths
        .par_iter()
        .filter_map(|(path, meta)| match process_one(path, meta, dither) {
            Ok(photo) => {
                println!("✓ Processed {}", meta.filename);
                Some(photo)
            }
            Err(e) => {
                eprintln!("✗ Error {}: {}", meta.filename, e);
                None
            }
        })
        .collect();

    photos.sort_by(|a, b| {
        let date_a = parse_date(&a.meta.date);
        let date_b = parse_date(&b.meta.date);
        date_b.cmp(&date_a)
    });

    Ok(photos)
}

fn parse_date(date_str: &str) -> (u32, u32, u32) {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let day = parts[0].parse().unwrap_or(0);
        let month = parts[1].parse().unwrap_or(0);
        let year = parts[2].parse().unwrap_or(0);
        (year, month, day)
    } else {
        (0, 0, 0)
    }
}

fn create_dirs() -> Result<()> {
    fs::create_dir_all("public/assets/dithered")?;
    fs::create_dir_all("public/assets/full")?;
    Ok(())
}

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            matches!(
                e.to_lowercase().as_str(),
                "jpg" | "jpeg" | "png" | "tiff" | "tif"
            )
        })
        .unwrap_or(false)
}

fn process_one(path: &Path, meta: &PhotoMetadata, dither: bool) -> Result<Photo> {
    let mut img = image::open(path).context("Failed to open image.")?;

    img = apply_exif_orientation(img, path);

    let (w, h) = (img.width(), img.height());

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid filename."))?;

    let thumb_filename = format!("{}.webp", stem);
    let full_filename = format!("{}.jpeg", stem);

    let thumb = create_thumbnail(&img, dither)?;
    thumb.save_with_format(
        format!("public/assets/dithered/{}", thumb_filename),
        ImageFormat::WebP,
    )?;

    img.save_with_format(
        format!("public/assets/full/{}", full_filename),
        ImageFormat::Jpeg,
    )?;

    Ok(Photo {
        original: meta.filename.clone(),
        thumb_filename,
        full_filename,
        meta: meta.clone(),
        width: w,
        height: h,
    })
}

fn apply_exif_orientation(img: DynamicImage, path: &Path) -> DynamicImage {
    let orientation = get_orientation(path).unwrap_or(1);

    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

fn get_orientation(path: &Path) -> Option<u32> {
    let file = fs::File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let exif_reader = Reader::new();
    let exif = exif_reader.read_from_container(&mut reader).ok()?;

    exif.get_field(Tag::Orientation, In::PRIMARY)
        .and_then(|field| field.value.get_uint(0))
}

fn create_thumbnail(img: &DynamicImage, apply_dither: bool) -> Result<DynamicImage> {
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
