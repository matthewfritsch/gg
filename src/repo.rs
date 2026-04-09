use anyhow::{Context, Result};
use git2::Repository;
use std::path::Path;

/// Open the git repository at or above the given path.
pub fn open() -> Result<Repository> {
    let repo = Repository::discover(".")
        .context("not a gg/git repository (or any parent up to mount point)")?;
    Ok(repo)
}

/// Initialize a new repository at the given path, or the current directory.
pub fn init(path: Option<&str>) -> Result<Repository> {
    let target = path.unwrap_or(".");
    let target = Path::new(target);

    let repo = if target.join(".git").exists() {
        // Already a git repo — just open it
        Repository::open(target)
            .context("failed to open existing git repository")?
    } else {
        Repository::init(target)
            .context("failed to initialize git repository")?
    };

    // Create .gg metadata directory
    let gg_dir = target.join(".gg");
    if !gg_dir.exists() {
        std::fs::create_dir_all(&gg_dir)
            .context("failed to create .gg directory")?;
    }

    Ok(repo)
}
