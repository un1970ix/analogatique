use crate::config::Config;
use crate::images::Photo;
use anyhow::Result;
use std::fs;
use tera::{Context, Tera};

pub fn create_site(config: &Config, photos: &[Photo]) -> Result<()> {
    fs::create_dir_all("public")?;

    let tera = load_templates()?;
    let ctx = build_context(config, photos);

    fs::write("public/index.html", tera.render("index.html", &ctx)?)?;
    fs::write("public/styles.css", tera.render("styles.css", &ctx)?)?;

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
