use super::PhotoMetadata;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn merge_metadata(
    existing: HashMap<String, PhotoMetadata>,
    photos_dir: &Path,
) -> Result<HashMap<String, PhotoMetadata>> {
    let mut merged = HashMap::new();

    let mut actual_files = std::collections::HashSet::new();
    for entry in fs::read_dir(photos_dir)? {
        let path = entry?.path();
        if is_image(&path) {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                actual_files.insert(filename.to_string());
            }
        }
    }

    let mut removed_count = 0;
    let mut preserved_count = 0;

    for (filename, metadata) in &existing {
        if actual_files.contains(filename) {
            merged.insert(filename.clone(), metadata.clone());
            preserved_count += 1;
        } else {
            println!("Removing metadata for deleted file: {}", filename);
            removed_count += 1;
        }
    }

    let mut new_files = Vec::new();
    for filename in &actual_files {
        if !merged.contains_key(filename) {
            new_files.push(filename.clone());
        }
    }

    let new_files_count = new_files.len();

    if !new_files.is_empty() {
        println!(
            "Found {} new files, extracting metadata...",
            new_files_count
        );

        for filename in new_files {
            let path = photos_dir.join(&filename);
            match super::exif::extract_from_file(&path, &filename) {
                Ok(meta) => {
                    println!("✓ Extracted metadata for: {}", filename);
                    merged.insert(filename, meta);
                }
                Err(e) => {
                    eprintln!("Could not extract Exif from {}: {}", filename, e);
                    merged.insert(
                        filename.clone(),
                        PhotoMetadata {
                            filename: filename.clone(),
                            date: "01-01-1970".to_string(),
                            name: None,
                            camera: None,
                            film: None,
                            lens: None,
                            location: None,
                            notes: None,
                        },
                    );
                }
            }
        }
    } else {
        println!("✓ No new files found");
    }

    println!("Metadata summary:");
    if preserved_count > 0 {
        println!("  - Existing entries preserved: {}", preserved_count);
    }
    if new_files_count > 0 {
        println!("  - New entries added: {}", new_files_count);
    }
    if removed_count > 0 {
        println!("  - Obsolete entries removed: {}", removed_count);
    }

    Ok(merged)
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
