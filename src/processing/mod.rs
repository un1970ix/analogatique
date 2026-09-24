pub mod dither;
mod formats;
mod resize;

use crate::config::Config;
use crate::metadata::PhotoMetadata;
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Photo {
    pub original: String,
    pub thumb_filename: String,
    pub full_filename: String,
    pub meta: PhotoMetadata,
    pub width: u32,
    pub height: u32,
}

pub fn process_all(
    config: &Config,
    metadata: &HashMap<String, PhotoMetadata>,
) -> Result<Vec<Photo>> {
    formats::process_all_images(config, metadata)
}
