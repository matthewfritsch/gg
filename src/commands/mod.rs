mod branch;
mod commit;
mod describe;
mod diff;
mod init;
mod log;
mod new;
mod passthrough;
mod status;

use crate::cli::{Cli, Command};
use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {
    let simple = cli.simple;
    match cli.command {
        None => default_view(simple),
        Some(Command::Init { path }) => init::run(path.as_deref()),
        Some(Command::Status | Command::St) => status::run(simple),
        Some(Command::Log { n }) => log::run(n, simple),
        Some(Command::Diff { paths, staged }) => diff::run(&paths, &staged, simple),
        Some(Command::Commit { message }) => commit::run(message.as_deref()),
        Some(Command::Describe { message } | Command::Desc { message }) => {
            describe::run(message.as_deref())
        }
        Some(Command::New { revision, message }) => {
            new::run(revision.as_deref(), message.as_deref())
        }
        Some(Command::Branch { action }) => branch::run(action, simple),
        Some(Command::External(args)) => passthrough::run(&args),
    }
}

fn default_view(simple: bool) -> Result<()> {
    if crate::repo::open().is_err() {
        println!("Not in a git repository. Run `gg init` to create one.");
        return Ok(());
    }
    log::run(10, simple)?;
    println!();
    status::run(simple)
}
