use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use yaml_rust::{YamlEmitter, YamlLoader};

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
    let root = YamlLoader::load_from_str(&orig)
        .with_context(|| format!("could not load yaml from: {}", args.yaml_file.display()))?;
    // write
    {
        let mut content = String::new();
        let mut emitter = YamlEmitter::new(&mut content);
        emitter
            .dump(&root[0])
            .with_context(|| "could not write yaml into string")?;
        println!("{}", &content);
    }
    Ok(())
}
