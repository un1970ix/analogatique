use super::{Photo, resize};
use crate::config::Config;
use crate::metadata::{PhotoMetadata, exif};
use anyhow::{Context, Result};
use image::ImageFormat;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn process_all_images(
    config: &Config,
    metadata: &HashMap<String, PhotoMetadata>,
) -> Result<Vec<Photo>> {
    let output_path = &config.output.path;
    let dither = config.dithering.enabled;

    let assets_dir = format!("{output_path}/assets");
    if Path::new(&assets_dir).exists() {
        fs::remove_dir_all(&assets_dir)?;
    }
    fs::create_dir_all(format!("{output_path}/assets/thumbnail"))?;
    fs::create_dir_all(format!("{output_path}/assets/full"))?;

    let mut image_paths = Vec::new();
    let mut unlisted = Vec::new();
    let photos_dir = Path::new("photos");

    if !photos_dir.exists() {
        return Err(anyhow::anyhow!(
            "photos/ directory not found. Use analogatique init or create the required directory manually."
        ));
    }

    for entry in fs::read_dir(photos_dir)? {
        let path = entry?.path();

        if is_image(&path)
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
        {
            match metadata.get(name) {
                Some(meta) => image_paths.push((path.clone(), meta.clone())),
                None => unlisted.push(name.to_string()),
            }
        }
    }

    if !unlisted.is_empty() {
        unlisted.sort();
        eprintln!(
            "warning: {} photo(s) in photos/ have no entry in metadata.txt and were skipped: {}",
            unlisted.len(),
            unlisted.join(", ")
        );
        eprintln!("         run 'analogatique extract-metadata' to add them");
    }

    image_paths.sort_by(|(a, _), (b, _)| a.file_name().cmp(&b.file_name()));

    let jobs = assign_output_names(&image_paths);

    let results: Vec<Result<Photo>> = jobs
        .par_iter()
        .map(|(path, meta, slug)| {
            let processed = process_one(path, meta, slug, dither, output_path);
            match &processed {
                Ok(_) => println!("✓ Processed {}", meta.filename),
                Err(e) => eprintln!("✗ Error processing {}: {e}", meta.filename),
            }
            processed
        })
        .collect();

    let failed = results.iter().filter(|r| r.is_err()).count();
    let mut photos: Vec<Photo> = results.into_iter().flatten().collect();

    if failed > 0 {
        return Err(anyhow::anyhow!(
            "{failed} of {} photo(s) could not be processed",
            jobs.len()
        ));
    }

    photos.sort_by(|a, b| {
        parse_date(&b.meta.date)
            .cmp(&parse_date(&a.meta.date))
            .then_with(|| a.original.cmp(&b.original))
    });

    Ok(photos)
}

fn assign_output_names(
    images: &[(PathBuf, PhotoMetadata)],
) -> Vec<(PathBuf, PhotoMetadata, String)> {
    let mut used: HashSet<String> = HashSet::new();
    let mut jobs = Vec::with_capacity(images.len());

    for (path, meta) in images {
        let stem = sanitize(path.file_stem().and_then(|s| s.to_str()).unwrap_or("photo"));

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("img")
            .to_lowercase();

        let mut slug = stem.clone();

        if used.contains(&slug) {
            slug = format!("{stem}-{extension}");
            let mut counter = 2;
            while used.contains(&slug) {
                slug = format!("{stem}-{extension}-{counter}");
                counter += 1;
            }
        }

        used.insert(slug.clone());
        jobs.push((path.clone(), meta.clone(), slug));
    }

    jobs
}

fn sanitize(raw: &str) -> String {
    let mut cleaned = String::with_capacity(raw.len());

    for c in raw.chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
            cleaned.push(c);
        } else if !cleaned.ends_with('-') {
            cleaned.push('-');
        }
    }

    let trimmed = cleaned.trim_matches(['-', '.'].as_slice());

    if trimmed.is_empty() {
        "photo".to_string()
    } else {
        trimmed.to_string()
    }
}

fn process_one(
    path: &Path,
    meta: &PhotoMetadata,
    slug: &str,
    dither: bool,
    output_path: &str,
) -> Result<Photo> {
    let mut img = image::open(path).context("Failed to open image")?;

    img = exif::apply_orientation(img, path);

    let thumb_filename = format!("{slug}.webp");
    let full_filename = format!("{slug}.jpeg");

    let thumb = resize::create_thumbnail(&img, dither);
    let (w, h) = (thumb.width(), thumb.height());
    thumb.save_with_format(
        format!("{output_path}/assets/thumbnail/{thumb_filename}"),
        ImageFormat::WebP,
    )?;

    img.save_with_format(
        format!("{output_path}/assets/full/{full_filename}"),
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

#[cfg(test)]
mod tests {
    use super::{assign_output_names, parse_date};
    use crate::metadata::PhotoMetadata;
    use std::path::PathBuf;

    fn meta(filename: &str) -> PhotoMetadata {
        PhotoMetadata {
            filename: filename.to_string(),
            date: "01-01-2024".to_string(),
            name: None,
            camera: None,
            film: None,
            lens: None,
            location: None,
            notes: None,
        }
    }

    fn names(files: &[&str]) -> Vec<String> {
        let images: Vec<(PathBuf, PhotoMetadata)> = files
            .iter()
            .map(|f| (PathBuf::from(format!("photos/{f}")), meta(f)))
            .collect();

        assign_output_names(&images)
            .into_iter()
            .map(|(_, _, slug)| slug)
            .collect()
    }

    #[test]
    fn same_stem_across_formats_does_not_collide() {
        let slugs = names(&["photo.jpg", "photo.png", "photo.tiff"]);
        assert_eq!(slugs, vec!["photo", "photo-png", "photo-tiff"]);

        let mut unique = slugs.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), slugs.len(), "output names collided");
    }

    #[test]
    fn distinct_stems_keep_their_plain_name() {
        assert_eq!(names(&["a.jpg", "b.jpg"]), vec!["a", "b"]);
    }

    #[test]
    fn repeated_stem_and_extension_gets_a_counter() {
        let images: Vec<(PathBuf, PhotoMetadata)> = ["x.jpg", "x.jpg", "x.jpg"]
            .iter()
            .map(|f| (PathBuf::from(format!("photos/{f}")), meta(f)))
            .collect();

        let slugs: Vec<String> = assign_output_names(&images)
            .into_iter()
            .map(|(_, _, slug)| slug)
            .collect();

        assert_eq!(slugs, vec!["x", "x-jpg", "x-jpg-2"]);
    }

    #[test]
    fn dates_sort_newest_first() {
        let mut dates = ["01-01-2024", "15-06-2023", "20-06-2023"];
        dates.sort_by_key(|d| std::cmp::Reverse(parse_date(d)));
        assert_eq!(dates, ["01-01-2024", "20-06-2023", "15-06-2023"]);
    }

    #[test]
    fn malformed_dates_do_not_panic() {
        assert_eq!(parse_date("nonsense"), (0, 0, 0));
        assert_eq!(parse_date("aa-bb-cccc"), (0, 0, 0));
    }
}

#[cfg(test)]
mod sanitize_tests {
    use super::sanitize;

    #[test]
    fn spaces_become_single_dashes() {
        assert_eq!(sanitize("a  b"), "a-b");
        assert_eq!(sanitize("my photo 2024"), "my-photo-2024");
    }

    #[test]
    fn unicode_letters_are_kept() {
        assert_eq!(sanitize("café été"), "café-été");
    }

    #[test]
    fn leading_dots_are_stripped_so_output_is_not_hidden() {
        assert_eq!(sanitize(".hidden"), "hidden");
        assert!(!sanitize(".hidden").starts_with('.'));
    }

    #[test]
    fn names_that_sanitise_to_nothing_get_a_fallback() {
        assert_eq!(sanitize("..."), "photo");
        assert_eq!(sanitize("   "), "photo");
    }

    #[test]
    fn safe_names_are_untouched() {
        assert_eq!(sanitize("photo_01-a.raw"), "photo_01-a.raw");
    }
}
