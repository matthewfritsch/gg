mod cli;
mod commands;
mod editor;
mod repo;

use anyhow::Result;
use cli::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli)
}
