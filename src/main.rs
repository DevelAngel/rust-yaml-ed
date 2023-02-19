use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use yaml_ed::yaml::YamlLoader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Yaml file
    #[arg(value_name = "FILE")]
    yaml_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let orig: String = std::fs::read_to_string(&args.yaml_file)
        .with_context(|| format!("could not read file: {}", args.yaml_file.display()))?;
    let mut content = String::new();
    let mut loader = YamlLoader::new(&mut content);
    loader.load_from_str(&orig);
    println!("{}", &content);
    Ok(())
}
