use anyhow::Result;
use std::fs;
use std::path::Path;

const DEFAULT_INDEX_TEMPLATE: &str = include_str!("../../templates/index.html");
const DEFAULT_STYLES_TEMPLATE: &str = include_str!("../../templates/styles.css");

pub fn ensure_templates() -> Result<()> {
    let templates_dir = Path::new("templates");

    let index_path = templates_dir.join("index.html");
    if !index_path.exists() {
        fs::write(index_path, DEFAULT_INDEX_TEMPLATE)?;
        println!("✓ Created templates/index.html");
    }

    let styles_path = templates_dir.join("styles.css");
    if !styles_path.exists() {
        fs::write(styles_path, DEFAULT_STYLES_TEMPLATE)?;
        println!("✓ Created templates/styles.css");
    }

    Ok(())
}
