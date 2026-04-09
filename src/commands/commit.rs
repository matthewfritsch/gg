use anyhow::{Context, Result};
use git2::StatusOptions;

pub fn run(message: Option<&str>) -> Result<()> {
    let repo = crate::repo::open()?;

    // Check if there are any changes to commit
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    if statuses.is_empty() {
        println!("Nothing to commit — working copy is clean");
        return Ok(());
    }

    let message = match message {
        Some(m) => m.to_string(),
        None => crate::editor::edit_message("")?,
    };

    // Stage everything (like jj — no manual staging)
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    let sig = repo.signature()
        .context("failed to get default signature — configure user.name and user.email in git")?;

    let parent = match repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };

    let parents: Vec<&git2::Commit> = parent.iter().collect();

    let oid = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &message,
        &tree,
        &parents,
    )?;

    let short = &oid.to_string()[..8];
    println!("Created commit {short}: {message}");

    Ok(())
}
