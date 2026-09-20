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

    metadata.camera = camera_name(
        trimmed_exif_string(&exif, Tag::Make),
        trimmed_exif_string(&exif, Tag::Model),
    );

    if let Some(lens_model) = get_exif_string(&exif, Tag::LensModel) {
        metadata.lens = Some(lens_model.trim().to_string());
    } else if let Some(lens_make) = get_exif_string(&exif, Tag::LensMake) {
        metadata.lens = Some(lens_make.trim().to_string());
    }

    if let Some(date_str) = get_exif_string(&exif, Tag::DateTimeOriginal)
        .or_else(|| get_exif_string(&exif, Tag::DateTime))
        && let Some(formatted) = parse_exif_date(&date_str)
    {
        metadata.date = formatted;
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

fn camera_name(make: Option<String>, model: Option<String>) -> Option<String> {
    match (make, model) {
        (Some(make), Some(model)) if model.starts_with(&make) => Some(model),
        (Some(make), Some(model)) => Some(format!("{make} {model}")),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn trimmed_exif_string(exif: &exif::Exif, tag: Tag) -> Option<String> {
    get_exif_string(exif, tag)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
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

#[cfg(test)]
mod tests {
    use super::{camera_name, parse_exif_date};

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn camera_joins_make_and_model() {
        assert_eq!(camera_name(s("Canon"), s("AE-1")), s("Canon AE-1"));
    }

    #[test]
    fn camera_falls_back_to_whichever_tag_is_present() {
        assert_eq!(camera_name(s("Polaroid"), None), s("Polaroid"));
        assert_eq!(camera_name(None, s("SX-70")), s("SX-70"));
        assert_eq!(camera_name(None, None), None);
    }

    #[test]
    fn camera_does_not_repeat_make_already_in_model() {
        assert_eq!(
            camera_name(s("Canon"), s("Canon EOS 5D")),
            s("Canon EOS 5D")
        );
    }

    #[test]
    fn exif_date_is_reordered_to_day_month_year() {
        assert_eq!(parse_exif_date("2023:06:15 10:30:00"), s("15-06-2023"));
        assert_eq!(parse_exif_date("not a date"), None);
    }
}
