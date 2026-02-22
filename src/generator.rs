use crate::config::Config;
use crate::processing::Photo;
use anyhow::Result;
use serde::Serialize;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

const NUM_COLUMNS: usize = 3;
const METADATA_HEIGHT_ESTIMATE: f64 = 0.3;

#[derive(Serialize)]
struct PhotoEntry<'a> {
    #[serde(flatten)]
    photo: &'a Photo,
    order: usize,
}

pub fn create_site(config: &Config, photos: &[Photo]) -> Result<()> {
    let output_path = &config.output.path;
    fs::create_dir_all(output_path)?;

    let page_dir = format!("{}/page", output_path);
    if Path::new(&page_dir).exists() {
        fs::remove_dir_all(&page_dir)?;
    }

    let tera = load_templates()?;
    let per_page = config.display.photos_per_page;
    let total_count = photos.len();

    let ctx = build_context(config, photos, total_count, 1, 1, "");
    fs::write(
        format!("{}/styles.css", output_path),
        tera.render("styles.css", &ctx)?,
    )?;

    if per_page == 0 || photos.is_empty() {
        fs::write(
            format!("{}/index.html", output_path),
            tera.render("index.html", &ctx)?,
        )?;
    } else {
        let pages: Vec<&[Photo]> = photos.chunks(per_page).collect();
        let total_pages = pages.len();

        for (i, page_photos) in pages.iter().enumerate() {
            let page_num = i + 1;
            let base_path = if page_num == 1 { "" } else { "../../" };
            let mut ctx = build_context(
                config,
                page_photos,
                total_count,
                page_num,
                total_pages,
                base_path,
            );

            if page_num > 1 {
                ctx.insert("prev_url", &page_url(page_num - 1, page_num));
            }
            if page_num < total_pages {
                ctx.insert("next_url", &page_url(page_num + 1, page_num));
            }

            let html = tera.render("index.html", &ctx)?;

            if page_num == 1 {
                fs::write(format!("{}/index.html", output_path), html)?;
            } else {
                let dir = format!("{}/page/{}", output_path, page_num);
                fs::create_dir_all(&dir)?;
                fs::write(format!("{}/index.html", dir), html)?;
            }
        }
    }

    Ok(())
}

fn distribute_into_columns(photos: &[Photo]) -> Vec<Vec<PhotoEntry<'_>>> {
    let num_columns = NUM_COLUMNS.min(photos.len()).max(1);
    let mut columns: Vec<Vec<PhotoEntry>> = (0..num_columns).map(|_| Vec::new()).collect();
    let mut heights = vec![0.0_f64; num_columns];

    for (order, photo) in photos.iter().enumerate() {
        let shortest = heights
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let visual_height = photo.height as f64 / photo.width.max(1) as f64;
        heights[shortest] += visual_height + METADATA_HEIGHT_ESTIMATE;
        columns[shortest].push(PhotoEntry { photo, order });
    }

    columns
}

fn page_url(target: usize, current: usize) -> String {
    if current == 1 {
        format!("page/{}/", target)
    } else if target == 1 {
        "../../".to_string()
    } else {
        format!("../{}/", target)
    }
}

fn load_templates() -> Result<Tera> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);
    Ok(tera)
}

fn build_context(
    config: &Config,
    photos: &[Photo],
    total_count: usize,
    current_page: usize,
    total_pages: usize,
    base_path: &str,
) -> Context {
    let columns = distribute_into_columns(photos);

    let mut ctx = Context::new();
    ctx.insert("site", &config.site);
    ctx.insert("display", &config.display);
    ctx.insert("footer", &config.footer);
    ctx.insert("columns", &columns);
    ctx.insert("count", &total_count);
    ctx.insert("current_page", &current_page);
    ctx.insert("total_pages", &total_pages);
    ctx.insert("base_path", base_path);
    ctx
}
