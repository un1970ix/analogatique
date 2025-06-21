mod cli;
mod config;
mod generator;
mod metadata;
mod processing;
mod utils;

fn main() -> anyhow::Result<()> {
    cli::run()
}
