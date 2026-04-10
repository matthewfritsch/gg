use anyhow::Result;
use colored::Colorize;
use git2::StatusOptions;
use std::collections::{BTreeMap, HashSet};

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

    let non_new = vec![&modified, &deleted, &renamed];
    let added = collapse_new_dirs(&added, &non_new);
    let untracked = collapse_new_dirs(&untracked, &non_new);

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

/// Collapse paths that live in entirely-new directories into a single entry.
/// A directory is "entirely new" if no file under it appears in `non_new_paths`.
fn collapse_new_dirs(paths: &[String], non_new_paths: &[&Vec<String>]) -> Vec<String> {
    if paths.len() < 2 {
        return paths.to_vec();
    }

    let non_new: HashSet<&str> = non_new_paths
        .iter()
        .flat_map(|v| v.iter())
        .map(|s| s.as_str())
        .collect();

    // Group paths by parent directory
    let mut dir_files: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut top_level = Vec::new();
    for path in paths {
        match path.rfind('/') {
            Some(pos) => dir_files.entry(&path[..pos]).or_default().push(path),
            None => top_level.push(path.clone()),
        }
    }

    let mut result = top_level;

    for (dir, files) in &dir_files {
        let prefix = format!("{dir}/");
        let has_non_new = non_new.iter().any(|p| p.starts_with(&prefix));

        if files.len() >= 2 && !has_non_new {
            result.push(format!("{dir}/ ({} files)", files.len()));
        } else {
            result.extend(files.iter().map(|f| f.to_string()));
        }
    }

    result.sort();
    result
}
