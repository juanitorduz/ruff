/// Print the AST for a given Python file.
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueHint};
use rustpython_parser::parser;

use ruff::fs;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath, required = true)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let contents = fs::read_file(&cli.file)?;
    let python_ast = parser::parse_program(&contents, &cli.file.to_string_lossy())?;

    println!("{:#?}", python_ast);

    Ok(())
}
