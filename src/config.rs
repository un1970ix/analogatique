use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub site: Site,
    pub dithering: Dithering,
    pub display: Display,
    pub footer: Footer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Site {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dithering {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Display {
    #[serde(default)]
    pub hide_filenames: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Footer {
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub url: String,
    pub name: String,
}

pub fn load() -> Result<Config> {
    let content = fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&content)?)
}
