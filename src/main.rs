#![feature(iter_array_chunks)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rb_parser::ResourceBundleParser;

mod rb_parser;
mod zone_info;

#[derive(Parser)]
struct Cli {
    zone_info_file: PathBuf,

    /// Path to save details for each zone.
    #[clap(long)]
    defs_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let zone_info = ResourceBundleParser::parse_to_zone_info(args.zone_info_file)?;
    zone_info.write_to(args.defs_dir)?;

    Ok(())
}
