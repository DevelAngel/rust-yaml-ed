use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use serde_yaml::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Yaml file
    #[arg(value_name = "FILE")]
    yaml_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let yaml_content: String = std::fs::read_to_string(&args.yaml_file)
        .with_context(|| format!("could not read file: {}", args.yaml_file.display()))?;
    let object: Value = serde_yaml::from_str(&yaml_content).with_context(|| {
        format!(
            "fail to convert into yaml structure: {}",
            args.yaml_file.display()
        )
    })?;
    let yaml_content = serde_yaml::to_string(&object).with_context(|| {
        format!(
            "fail to convert yaml structure to string: {}",
            args.yaml_file.display()
        )
    })?;
    println!("{}", &yaml_content);
    Ok(())
}
