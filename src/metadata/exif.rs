use super::PhotoMetadata;
use anyhow::Result;
use exif::{In, Reader, Tag, Value};
use image::DynamicImage;
use std::fs;
use std::path::Path;

pub fn extract_from_file(path: &Path, filename: &str) -> Result<PhotoMetadata> {
    let exif = read_exif(path)?;

    let mut metadata = PhotoMetadata {
        filename: filename.to_string(),
        date: "01-01-1970".to_string(),
        name: None,
        camera: None,
        film: None,
        lens: None,
        location: None,
        notes: None,
    };

    if let Some(make) = get_exif_string(&exif, Tag::Make) {
        if let Some(model) = get_exif_string(&exif, Tag::Model) {
            metadata.camera = Some(format!("{} {}", make.trim(), model.trim()));
        }
    }

    if let Some(lens_model) = get_exif_string(&exif, Tag::LensModel) {
        metadata.lens = Some(lens_model.trim().to_string());
    } else if let Some(lens_make) = get_exif_string(&exif, Tag::LensMake) {
        metadata.lens = Some(lens_make.trim().to_string());
    }

    if let Some(date_str) = get_exif_string(&exif, Tag::DateTimeOriginal)
        .or_else(|| get_exif_string(&exif, Tag::DateTime))
    {
        if let Some(formatted) = parse_exif_date(&date_str) {
            metadata.date = formatted;
        }
    }

    Ok(metadata)
}

pub fn get_orientation(path: &Path) -> u32 {
    read_exif(path)
        .ok()
        .and_then(|exif| {
            exif.get_field(Tag::Orientation, In::PRIMARY)
                .and_then(|field| field.value.get_uint(0))
        })
        .unwrap_or(1)
}

pub fn apply_orientation(img: DynamicImage, path: &Path) -> DynamicImage {
    let orientation = get_orientation(path);

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

fn read_exif(path: &Path) -> Result<exif::Exif> {
    let file = fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let exif_reader = Reader::new();
    Ok(exif_reader.read_from_container(&mut reader)?)
}

fn get_exif_string(exif: &exif::Exif, tag: Tag) -> Option<String> {
    exif.get_field(tag, In::PRIMARY)
        .and_then(|field| match &field.value {
            Value::Ascii(vec) => vec.first().map(|v| String::from_utf8_lossy(v).to_string()),
            _ => field.display_value().to_string().into(),
        })
}

fn parse_exif_date(date_str: &str) -> Option<String> {
    let parts: Vec<&str> = date_str.split_whitespace().collect();
    if let Some(date_part) = parts.first() {
        let date_components: Vec<&str> = date_part.split(':').collect();
        if date_components.len() == 3 {
            return Some(format!(
                "{}-{}-{}",
                date_components[2], date_components[1], date_components[0]
            ));
        }
    }
    None
}
