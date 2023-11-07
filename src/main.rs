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
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let info = ResourceBundleParser::parse_to_zone_info(args.zone_info_file)?;

    Ok(())
}
