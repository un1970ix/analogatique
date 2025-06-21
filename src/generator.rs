use crate::config::Config;
use crate::processing::Photo;
use anyhow::Result;
use std::fs;
use tera::{Context, Tera};

pub fn create_site(config: &Config, photos: &[Photo]) -> Result<()> {
    let output_path = &config.output.path;
    fs::create_dir_all(output_path)?;

    let tera = load_templates()?;
    let ctx = build_context(config, photos);

    fs::write(
        format!("{}/index.html", output_path),
        tera.render("index.html", &ctx)?,
    )?;
    fs::write(
        format!("{}/styles.css", output_path),
        tera.render("styles.css", &ctx)?,
    )?;

    Ok(())
}

fn load_templates() -> Result<Tera> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);
    Ok(tera)
}

fn build_context(config: &Config, photos: &[Photo]) -> Context {
    let mut ctx = Context::new();
    ctx.insert("site", &config.site);
    ctx.insert("display", &config.display);
    ctx.insert("footer", &config.footer);
    ctx.insert("photos", photos);
    ctx.insert("count", &photos.len());
    ctx
}
