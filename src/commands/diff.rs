use anyhow::Result;
use colored::Colorize;
use git2::DiffOptions;

pub fn run(paths: &[String], _staged: &bool, simple: bool) -> Result<()> {
    let repo = crate::repo::open()?;

    let head_tree = match repo.head() {
        Ok(head) => {
            let commit = head.peel_to_commit()?;
            Some(commit.tree()?)
        }
        Err(_) => None,
    };

    let mut opts = DiffOptions::new();
    for path in paths {
        opts.pathspec(path);
    }

    let diff = repo.diff_tree_to_workdir_with_index(head_tree.as_ref(), Some(&mut opts))?;

    if diff.stats()?.files_changed() == 0 {
        println!("No changes");
        return Ok(());
    }

    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let content = std::str::from_utf8(line.content()).unwrap_or("");
        if simple {
            match line.origin() {
                '+' => print!("+{content}"),
                '-' => print!("-{content}"),
                ' ' => print!(" {content}"),
                _ => print!("{content}"),
            }
        } else {
            match line.origin() {
                '+' => print!("{}", format!("+{content}").green()),
                '-' => print!("{}", format!("-{content}").red()),
                ' ' => print!(" {content}"),
                'H' => print!("{}", content.cyan()),
                'F' => print!("{}", content.bold()),
                _ => print!("{content}"),
            }
        }
        true
    })?;

    Ok(())
}
