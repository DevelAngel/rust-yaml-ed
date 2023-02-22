use std::path;

use anyhow::{Context, Result};
use clap::Parser;

use yaml_ed::YamlEd;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Yaml file
    #[arg(value_name = "FILE")]
    yaml_file: path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ed = YamlEd::try_from(&args.yaml_file)
        .with_context(|| format!("could not load yaml from: {}", args.yaml_file.display()))?;
    println!("{}", ed);
    Ok(())
}
