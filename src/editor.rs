use anyhow::{bail, Context, Result};
use std::io::Write;

/// Open the user's editor to compose a message.
/// Resolution matches git's: `git var GIT_EDITOR` (which checks GIT_EDITOR,
/// core.editor, VISUAL, EDITOR in order), falling back to vi.
/// `initial` is pre-filled into the editor buffer.
/// Lines starting with `#` are stripped from the result.
pub fn edit_message(initial: &str) -> Result<String> {
    let editor = resolve_editor();

    let mut tmp = tempfile::NamedTempFile::new().context("failed to create temp file")?;
    write!(tmp, "{initial}\n\n# Write your message above.\n# Lines starting with '#' will be ignored.\n")?;
    tmp.flush()?;

    let path = tmp.path().to_path_buf();

    let path_str = path.display();
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} \"{path_str}\""))
        .status()
        .with_context(|| format!("failed to launch editor '{editor}'"))?;

    if !status.success() {
        bail!("editor exited with non-zero status");
    }

    let content = std::fs::read_to_string(&path).context("failed to read editor output")?;

    let message: String = content
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if message.is_empty() {
        bail!("empty message — aborting");
    }

    Ok(message)
}

fn resolve_editor() -> String {
    if let Ok(output) = std::process::Command::new("git")
        .args(["var", "GIT_EDITOR"])
        .output()
    {
        if output.status.success() {
            let editor = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !editor.is_empty() {
                return editor;
            }
        }
    }
    "vi".to_string()
}
