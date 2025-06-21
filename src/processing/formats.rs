use super::{Photo, resize};
use crate::config;
use crate::metadata::{PhotoMetadata, exif};
use anyhow::{Context, Result};
use image::ImageFormat;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn process_all_images(
    metadata: &HashMap<String, PhotoMetadata>,
    dither: bool,
) -> Result<Vec<Photo>> {
    let config = config::load()?;
    let output_path = &config.output.path;

    fs::create_dir_all(format!("{}/assets/thumbnail", output_path))?;
    fs::create_dir_all(format!("{}/assets/full", output_path))?;

    let mut image_paths = Vec::new();
    let photos_dir = Path::new("photos");

    if !photos_dir.exists() {
        return Err(anyhow::anyhow!(
            "photos/ directory not found. Use analogatique init or create the required directory manually."
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
        .filter_map(
            |(path, meta)| match process_one(path, meta, dither, output_path) {
                Ok(photo) => {
                    println!("✓ Processed {}", meta.filename);
                    Some(photo)
                }
                Err(e) => {
                    eprintln!("✗ Error processing {}: {}", meta.filename, e);
                    None
                }
            },
        )
        .collect();

    photos.sort_by(|a, b| {
        let date_a = parse_date(&a.meta.date);
        let date_b = parse_date(&b.meta.date);
        date_b.cmp(&date_a)
    });

    Ok(photos)
}

fn process_one(
    path: &Path,
    meta: &PhotoMetadata,
    dither: bool,
    output_path: &str,
) -> Result<Photo> {
    let mut img = image::open(path).context("Failed to open image")?;

    img = exif::apply_orientation(img, path);

    let (w, h) = (img.width(), img.height());

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;

    let thumb_filename = format!("{}.webp", stem);
    let full_filename = format!("{}.jpeg", stem);

    let thumb = resize::create_thumbnail(&img, dither)?;
    thumb.save_with_format(
        format!("{}/assets/thumbnail/{}", output_path, thumb_filename),
        ImageFormat::WebP,
    )?;

    img.save_with_format(
        format!("{}/assets/full/{}", output_path, full_filename),
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

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| {
            matches!(
                e.to_lowercase().as_str(),
                "jpg" | "jpeg" | "png" | "tif" | "tiff"
            )
        })
        .unwrap_or(false)
}
