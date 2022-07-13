use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::process;

use cli::app;
use cli::util::{is_exist, read_file};

fn run() -> Result<()> {
    let matches = app::build().get_matches_from(env::args_os());

    let path: PathBuf = match matches.value_of("path") {
        Some(path) => is_exist(path)?,
        None => anyhow::bail!("foo"),
    };

    let content = read_file(path)?;
    println!("{}", content);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
