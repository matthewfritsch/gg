use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gg", about = "A CLI git UI inspired by jj", version)]
pub struct Cli {
    /// Plain text output without colors or Unicode (machine/AI-readable)
    #[arg(long, global = true)]
    pub simple: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
#[command(allow_external_subcommands = true)]
pub enum Command {
    /// Initialize a new gg repository
    Init {
        /// Path to initialize (defaults to current directory)
        path: Option<String>,
    },

    /// Show working copy status
    Status,

    /// Show working copy status (alias for status)
    St,

    /// Show commit log with graph
    Log {
        /// Number of commits to show
        #[arg(short, long, default_value = "20")]
        n: usize,
    },

    /// Show working copy diff
    Diff {
        /// Paths to diff (defaults to all)
        paths: Vec<String>,
        staged: bool,
    },

    /// Commit all working copy changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Set or update the description of the current commit
    Describe {
        /// New commit message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Set or update the description of the current commit (alias for describe)
    Desc {
        /// New commit message
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Create a new empty change
    New {
        /// Revision to create the new change on top of
        revision: Option<String>,

        /// Message for the new change
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Manage branches
    Branch {
        #[command(subcommand)]
        action: BranchAction,
    },

    /// Pass through to git
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Subcommand)]
pub enum BranchAction {
    /// List all branches
    List,

    /// Create a new branch at HEAD
    Create {
        /// Branch name
        name: String,
    },

    /// Delete a branch
    Delete {
        /// Branch name
        name: String,
    },

    /// Move a branch to the current commit
    Set {
        /// Branch name
        name: String,
    },
}
