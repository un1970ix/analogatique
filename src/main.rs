mod config;
mod dither;
mod generator;
mod images;
mod metadata;

use anyhow::Result;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--version" || args[1] == "-v") {
        println!("{} {}", NAME, VERSION);
        return Ok(());
    }

    println!("analogatique · web gallery for you photos");

    let config = config::load()?;
    println!("✓ Loaded configuration for {}!", config.site.title);

    let metadata = metadata::load()?;
    println!("✓ Loaded metadata for {} photos.", metadata.len());

    let photos = images::process_all(&metadata, config.dithering.enabled)?;
    println!("✓ Processed {} photos.", photos.len());

    generator::create_site(&config, &photos)?;
    println!("✓ Generated website in the /public directory.");
    println!("Done! Your gallery is ready to deploy. :-D");

    Ok(())
}
