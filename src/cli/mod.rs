mod commands;

use anyhow::Result;
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "--version" | "-v" => {
            println!("{} {}", NAME, VERSION);
            Ok(())
        }
        "init" => commands::init(),
        "extract-metadata" => commands::extract_metadata(),
        "generate" => commands::generate(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!();
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("{} {}", NAME, VERSION);
    println!("web gallery for you photos");
    println!();
    println!("USAGE:");
    println!("    {} <COMMAND>", NAME);
    println!();
    println!("COMMANDS:");
    println!("    init              Initialize a new gallery project.");
    println!("    extract-metadata  Extract Exif data from photos to metadata.txt.");
    println!("    generate          Generate the static gallery website.");
    println!();
    println!("OPTIONS:");
    println!("    -v, --version     Print version information.");
}
