pub mod exif;
pub mod merger;
pub mod parser;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PhotoMetadata {
    pub filename: String,
    pub date: String,
    pub name: Option<String>,
    pub camera: Option<String>,
    pub film: Option<String>,
    pub lens: Option<String>,
    pub location: Option<String>,
    pub notes: Option<String>,
}

pub fn load() -> Result<HashMap<String, PhotoMetadata>> {
    parser::load_from_file("metadata.txt")
}

pub fn save(metadata: &HashMap<String, PhotoMetadata>) -> Result<()> {
    parser::save_to_file("metadata.txt", metadata)
}
