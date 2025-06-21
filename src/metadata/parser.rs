use super::PhotoMetadata;
use anyhow::Result;
use std::collections::HashMap;
use std::fs;

pub fn load_from_file(path: &str) -> Result<HashMap<String, PhotoMetadata>> {
    let content = fs::read_to_string(path)?;
    let mut map = HashMap::new();

    for line in content.lines().filter(|l| !l.trim().is_empty()) {
        let meta: PhotoMetadata = serde_json::from_str(line)?;
        map.insert(meta.filename.clone(), meta);
    }

    Ok(map)
}

pub fn save_to_file(path: &str, metadata: &HashMap<String, PhotoMetadata>) -> Result<()> {
    let mut lines = Vec::new();

    let mut entries: Vec<_> = metadata.iter().collect();
    entries.sort_by_key(|(filename, _)| filename.as_str());

    for (_, meta) in entries {
        lines.push(serde_json::to_string(meta)?);
    }

    fs::write(path, lines.join("\n"))?;
    Ok(())
}
