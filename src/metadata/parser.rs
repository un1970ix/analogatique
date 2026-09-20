use super::PhotoMetadata;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

pub fn load_from_file(path: &str) -> Result<HashMap<String, PhotoMetadata>> {
    let content = fs::read_to_string(path)?;
    let mut map = HashMap::new();

    for (index, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let meta: PhotoMetadata = serde_json::from_str(line).with_context(|| {
            format!(
                "{} line {}: malformed JSON, refusing to continue so existing entries are not lost",
                path,
                index + 1
            )
        })?;

        map.insert(meta.filename.clone(), meta);
    }

    Ok(map)
}

pub fn save_to_file(path: &str, metadata: &HashMap<String, PhotoMetadata>) -> Result<()> {
    let mut entries: Vec<_> = metadata.iter().collect();
    entries.sort_by_key(|(filename, _)| filename.as_str());

    let mut lines = Vec::new();
    for (_, meta) in entries {
        lines.push(serde_json::to_string(meta)?);
    }

    let content = if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    };

    fs::write(path, content)?;
    Ok(())
}
