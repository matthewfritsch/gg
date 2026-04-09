use anyhow::Result;
use colored::Colorize;
use git2::StatusOptions;

pub fn run(simple: bool) -> Result<()> {
    let repo = crate::repo::open()?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(true);
    opts.renames_head_to_index(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    if statuses.is_empty() {
        println!("Working copy is clean");
        return Ok(());
    }

    let mut modified = Vec::new();
    let mut added = Vec::new();
    let mut deleted = Vec::new();
    let mut renamed = Vec::new();
    let mut untracked = Vec::new();

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("???").to_string();
        let status = entry.status();

        if status.is_index_renamed() || status.is_wt_renamed() {
            renamed.push(path);
        } else if status.is_wt_new() || status.is_index_new() {
            if status.is_wt_new() && !status.is_index_new() {
                untracked.push(path);
            } else {
                added.push(path);
            }
        } else if status.is_wt_modified() || status.is_index_modified() {
            modified.push(path);
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            deleted.push(path);
        }
    }

    if simple {
        for f in &modified {
            println!("M {f}");
        }
        for f in &added {
            println!("A {f}");
        }
        for f in &deleted {
            println!("D {f}");
        }
        for f in &renamed {
            println!("R {f}");
        }
        for f in &untracked {
            println!("? {f}");
        }
        return Ok(());
    }

    if !modified.is_empty() {
        println!("{}:", "Modified".blue().bold());
        for f in &modified {
            println!("  {} {f}", "M".blue());
        }
    }
    if !added.is_empty() {
        println!("{}:", "Added".green().bold());
        for f in &added {
            println!("  {} {f}", "A".green());
        }
    }
    if !deleted.is_empty() {
        println!("{}:", "Deleted".red().bold());
        for f in &deleted {
            println!("  {} {f}", "D".red());
        }
    }
    if !renamed.is_empty() {
        println!("{}:", "Renamed".cyan().bold());
        for f in &renamed {
            println!("  {} {f}", "R".cyan());
        }
    }
    if !untracked.is_empty() {
        println!("{}:", "Untracked".yellow().bold());
        for f in &untracked {
            println!("  {} {f}", "?".yellow());
        }
    }

    Ok(())
}
