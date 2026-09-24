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

        if !is_valid_date(&meta.date) {
            eprintln!(
                "warning: {} line {}: date \"{}\" is not DD-MM-YYYY and will sort last",
                path,
                index + 1,
                meta.date
            );
        }

        if let Some(previous) = map.insert(meta.filename.clone(), meta) {
            eprintln!(
                "warning: {} line {}: duplicate entry for {}, the later one wins",
                path,
                index + 1,
                previous.filename
            );
        }
    }

    Ok(map)
}

fn is_valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    parts.len() == 3
        && parts[0].len() == 2
        && parts[1].len() == 2
        && parts[2].len() == 4
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
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

#[cfg(test)]
mod tests {
    use super::{load_from_file, save_to_file};
    use crate::metadata::PhotoMetadata;
    use std::collections::HashMap;
    use std::path::PathBuf;

    struct TempFile(PathBuf);

    impl TempFile {
        fn new(name: &str, contents: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("analogatique-{name}-{}", std::process::id()));
            std::fs::write(&path, contents).unwrap();
            TempFile(path)
        }

        fn path(&self) -> &str {
            self.0.to_str().unwrap()
        }

        fn read(&self) -> String {
            std::fs::read_to_string(&self.0).unwrap()
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    const CURATED: &str = concat!(
        r#"{"filename":"a.jpg","date":"15-06-2023","name":"Sunset","notes":"keep me"}"#,
        "\n",
        r#"{"filename":"b.jpg","date":"20-06-2023","name":"Vista"}"#,
        "\n"
    );

    #[test]
    fn malformed_line_is_an_error_naming_the_line_number() {
        let f = TempFile::new("bad", &format!("{CURATED}{{\"filename\": broken}}\n"));
        let err = load_from_file(f.path()).unwrap_err().to_string();
        assert!(err.contains("line 3"), "error did not name the line: {err}");
    }

    #[test]
    fn malformed_line_leaves_the_file_untouched() {
        let original = format!("{CURATED}{{\"filename\": broken}}\n");
        let f = TempFile::new("intact", &original);
        assert!(load_from_file(f.path()).is_err());
        assert_eq!(f.read(), original, "curated metadata was modified");
    }

    #[test]
    fn blank_lines_are_skipped() {
        let f = TempFile::new("blank", &format!("\n{CURATED}\n\n"));
        assert_eq!(load_from_file(f.path()).unwrap().len(), 2);
    }

    #[test]
    fn round_trip_preserves_entries_and_ends_with_a_newline() {
        let f = TempFile::new("round", CURATED);
        let loaded = load_from_file(f.path()).unwrap();

        save_to_file(f.path(), &loaded).unwrap();
        assert!(f.read().ends_with('\n'), "file does not end with a newline");
        assert_eq!(load_from_file(f.path()).unwrap(), loaded);
    }

    #[test]
    fn empty_metadata_writes_an_empty_file() {
        let f = TempFile::new("empty", CURATED);
        save_to_file(f.path(), &HashMap::<String, PhotoMetadata>::new()).unwrap();
        assert_eq!(f.read(), "");
    }
}
