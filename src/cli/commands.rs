use crate::{config, generator, metadata, processing, utils};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate() -> Result<()> {
    println!("analogatique · web gallery for you photos");

    let config = config::load()?;
    println!("✓ Loaded configuration for {}!", config.site.title);

    let metadata = metadata::load()?;
    println!("✓ Loaded metadata for {} photos.", metadata.len());

    let photos = processing::process_all(&metadata, config.dithering.enabled)?;
    println!("✓ Processed {} photos.", photos.len());

    generator::create_site(&config, &photos)?;
    println!(
        "✓ Generated website in this directory: {}/",
        config.output.path
    );
    println!("Done! Your gallery is ready to deploy.");

    Ok(())
}

pub fn init() -> Result<()> {
    println!("Initializing new analogatique gallery...");

    fs::create_dir_all("photos")?;
    fs::create_dir_all("templates")?;

    if !Path::new("config.toml").exists() {
        fs::write("config.toml", DEFAULT_CONFIG)?;
        println!("✓ Created the config.toml file.");
    } else {
        println!("config.toml already exists, skipping...");
    }

    if !Path::new("metadata.txt").exists() {
        fs::write("metadata.txt", "")?;
        println!("✓ Created the metadata.txt file.");
    }

    utils::fs::ensure_templates()?;

    println!("✓ Gallery initialized!");
    println!("\nNext steps:");
    println!("1. Add your photos to the photos/ directory.");
    println!("2. Edit config.toml with your information.");
    println!("3. Run 'analogatique extract-metadata' or 'analogatique generate'.");

    Ok(())
}

pub fn extract_metadata() -> Result<()> {
    println!("Checking for metadata updates...");

    let photos_dir = Path::new("photos");
    if !photos_dir.exists() {
        return Err(anyhow::anyhow!(
            "No photos/ directory found. Run 'analogatique init' or create it yourself."
        ));
    }

    let existing = metadata::load().unwrap_or_default();
    let existing_count = existing.len();

    let existing_copy = existing.clone();

    let merged = metadata::merger::merge_metadata(existing, photos_dir)?;

    if merged.len() != existing_count || merged != existing_copy {
        metadata::save(&merged)?;
        println!("✓ Metadata file updated. Total entries: {}", merged.len());
    } else {
        println!("✓ No changes needed. Metadata is up-to-date.");
    }

    Ok(())
}

const DEFAULT_CONFIG: &str = r#"[site]
title = ""
subtitle = ""
description = ""
author = ""

[dithering]
enabled = false

[display]
hide_filenames = true

[output]
path = "public"

[footer]
links = [
    { name = "", url = "" },
    { name = "", url = "" }
]
"#;
