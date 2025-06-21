pub mod dither;
mod formats;
mod resize;

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

pub fn process_all(metadata: &HashMap<String, PhotoMetadata>, dither: bool) -> Result<Vec<Photo>> {
    formats::process_all_images(metadata, dither)
}
