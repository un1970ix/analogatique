use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    let content = fs::read_to_string("metadata.txt")?;
    let mut map = HashMap::new();

    for line in content.lines().filter(|l| !l.trim().is_empty()) {
        let meta: PhotoMetadata = serde_json::from_str(line)?;
        map.insert(meta.filename.clone(), meta);
    }

    Ok(map)
}
