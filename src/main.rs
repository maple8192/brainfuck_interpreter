use std::{
    fs::read_to_string,
    io::{stdin, stdout},
    path::PathBuf,
};

use anyhow::anyhow;
use clap::{command, Parser};

/// Represents a command line argument.
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Source code to be interpreted.
    source: Option<String>,

    /// File path to the source code.
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source = get_source(&args)?;
    brainfuck_interpreter::run(&source, stdin().lock(), stdout())?;
    Ok(())
}

/// Returns the source code from the command line arguments or reads it from the specified file.
///
/// # Errors
///
/// Returns an error if neither source code nor file path is provided, or if the file cannot be read.
fn get_source(args: &Args) -> anyhow::Result<String> {
    if let Some(src) = &args.source {
        Ok(src.clone())
    } else if let Some(path) = &args.file {
        Ok(read_to_string(path)?)
    } else {
        Err(anyhow!("No source code provided."))
    }
}
