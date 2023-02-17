use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use yaml_peg::{dump, parse, repr::RcRepr};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Yaml file
    #[arg(value_name = "FILE")]
    yaml_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // read
    let orig: String = std::fs::read_to_string(&args.yaml_file)
        .with_context(|| format!("could not read file: {}", args.yaml_file.display()))?;
    let root = parse::<RcRepr>(&orig)
        .with_context(|| format!("could not load yaml from: {}", args.yaml_file.display()))?;
    // write
    {
        let content = dump(&root, &[]);
        println!("{}", &content);
    }
    Ok(())
}
