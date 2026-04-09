use anyhow::{Context, Result};
use colored::Colorize;

pub fn run(n: usize, simple: bool) -> Result<()> {
    let repo = crate::repo::open()?;

    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => {
            println!("No commits yet");
            return Ok(());
        }
    };

    let head_oid = head.target().context("HEAD has no target")?;
    let head_name = head.shorthand().unwrap_or("HEAD").to_string();

    let mut revwalk = repo.revwalk()?;
    revwalk.push(head_oid)?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;

    let commits: Vec<_> = revwalk
        .take(n)
        .filter_map(|oid| oid.ok())
        .filter_map(|oid| repo.find_commit(oid).ok())
        .collect();

    if commits.is_empty() {
        println!("No commits yet");
        return Ok(());
    }

    if simple {
        for (i, commit) in commits.iter().enumerate() {
            let oid = commit.id();
            let short_id = &oid.to_string()[..8];
            let message = commit.summary().unwrap_or("(no message)");
            let author = commit.author();
            let author_name = author.name().unwrap_or("Unknown");
            let is_head = i == 0;
            let head_marker = if is_head { " @" } else { "" };
            println!("{short_id} {author_name} {message}{head_marker}");
        }
        return Ok(());
    }

    for (i, commit) in commits.iter().enumerate() {
        let is_head = i == 0;
        let oid = commit.id();
        let short_id = &oid.to_string()[..8];
        let message = commit.summary().unwrap_or("(no message)");
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown");
        let time = format_relative_time(commit.time().seconds());

        let graph_char = if is_head { "◆" } else { "○" };

        if is_head {
            print!("{} ", graph_char.magenta().bold());
            print!("{} ", short_id.magenta().bold());
            print!("{}", head_name.magenta());
            println!(
                " {} {} {}",
                author_name.dimmed(),
                "·".dimmed(),
                time.dimmed()
            );
            let bar = "│".magenta();
            println!("{bar} {}", message.bold());
        } else {
            print!("{} ", graph_char.blue());
            print!("{} ", short_id.yellow());
            println!(
                "{} {} {}",
                author_name.dimmed(),
                "·".dimmed(),
                time.dimmed()
            );
            println!("{} {message}", "│".blue());
        }

        if commit.parent_count() > 1 {
            let parents: Vec<String> = (0..commit.parent_count())
                .filter_map(|i| commit.parent_id(i).ok())
                .map(|oid| oid.to_string()[..8].to_string())
                .collect();
            println!(
                "{} {}",
                "├─╮".blue(),
                format!("merge of {}", parents.join(", ")).dimmed()
            );
        }
    }

    Ok(())
}

fn format_relative_time(timestamp: i64) -> String {
    let now = chrono::Utc::now().timestamp();
    let diff = now - timestamp;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        let mins = diff / 60;
        format!("{mins}m ago")
    } else if diff < 86400 {
        let hours = diff / 3600;
        format!("{hours}h ago")
    } else if diff < 604800 {
        let days = diff / 86400;
        format!("{days}d ago")
    } else if diff < 2592000 {
        let weeks = diff / 604800;
        format!("{weeks}w ago")
    } else if diff < 31536000 {
        let months = diff / 2592000;
        format!("{months}mo ago")
    } else {
        let years = diff / 31536000;
        format!("{years}y ago")
    }
}
