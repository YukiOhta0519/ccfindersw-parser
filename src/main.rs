use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, stdout};
use anyhow::{Context, Result};
use serde_json;
mod parser;
use parser::parse_ccfinder_output;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Path to the CCFinderSW output file
    #[arg(short, long)]
    input: PathBuf,
    
    /// Path to the output JSON file (if not specified, output to standard output)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Whether to output formatted JSON
    #[arg(short, long, default_value_t = false)]
    pretty: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = parse_ccfinder_output(args.input).with_context(|| "Failed to parse CCFinderSW output")?;

    if let Some(output_path) = args.output {
        let file = File::create(&output_path)?;
        let writer = BufWriter::new(file);
        if args.pretty {
            serde_json::to_writer_pretty(writer, &output)?;
        } else {
            serde_json::to_writer(writer, &output)?;
        }
    } else {
        let stdout = stdout();
        let writer = BufWriter::new(stdout.lock());
        if args.pretty {
            serde_json::to_writer_pretty(writer, &output)?;
        } else {
            serde_json::to_writer(writer, &output)?;
        }
    }
    Ok(())     
}
