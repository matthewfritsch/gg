use std::path::PathBuf;
use std::process::Command;

struct TestRepo {
    dir: tempfile::TempDir,
}

impl TestRepo {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        Self { dir }
    }

    fn path(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    fn gg(&self, args: &[&str]) -> std::process::Output {
        let bin = env!("CARGO_BIN_EXE_gg");
        Command::new(bin)
            .args(args)
            .current_dir(self.dir.path())
            .env("GIT_AUTHOR_NAME", "Test")
            .env("GIT_AUTHOR_EMAIL", "test@test.com")
            .env("GIT_COMMITTER_NAME", "Test")
            .env("GIT_COMMITTER_EMAIL", "test@test.com")
            .output()
            .expect("failed to run gg")
    }

    fn gg_ok(&self, args: &[&str]) -> String {
        let output = self.gg(args);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        assert!(
            output.status.success(),
            "gg {:?} failed:\nstdout: {stdout}\nstderr: {stderr}",
            args
        );
        stdout
    }

    fn write_file(&self, name: &str, content: &str) {
        std::fs::write(self.path().join(name), content).expect("failed to write file");
    }
}

// ── Init ──

#[test]
fn test_init_creates_git_and_gg_dirs() {
    let repo = TestRepo::new();
    let out = repo.gg_ok(&["init"]);
    assert!(out.contains("Initialized"));
    assert!(repo.path().join(".git").exists());
    assert!(repo.path().join(".gg").exists());
}

#[test]
fn test_init_existing_git_repo() {
    let repo = TestRepo::new();
    Command::new("git")
        .args(["init"])
        .current_dir(repo.path())
        .output()
        .unwrap();
    let out = repo.gg_ok(&["init"]);
    assert!(out.contains("Initialized"));
    assert!(repo.path().join(".gg").exists());
}

// ── Status ──

#[test]
fn test_status_clean() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    let out = repo.gg_ok(&["status"]);
    assert!(out.contains("clean"));
}

#[test]
fn test_status_shows_untracked() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("hello.txt", "hello");
    let out = repo.gg_ok(&["status"]);
    assert!(out.contains("hello.txt"));
}

#[test]
fn test_st_alias() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("hello.txt", "hello");
    let out = repo.gg_ok(&["st"]);
    assert!(out.contains("hello.txt"));
}

#[test]
fn test_status_simple() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("a.txt", "a");
    let out = repo.gg_ok(&["--simple", "status"]);
    assert!(out.contains("? a.txt"));
}

// ── Commit ──

#[test]
fn test_commit_all_changes() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("file.txt", "content");
    let out = repo.gg_ok(&["commit", "-m", "first"]);
    assert!(out.contains("Created commit"));
    assert!(out.contains("first"));
}

#[test]
fn test_commit_no_changes() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("file.txt", "content");
    repo.gg_ok(&["commit", "-m", "first"]);
    let out = repo.gg_ok(&["commit", "-m", "second"]);
    assert!(out.contains("clean"));
}

// ── Log ──

#[test]
fn test_log_shows_commits() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first commit"]);
    let out = repo.gg_ok(&["log"]);
    assert!(out.contains("first commit"));
}

#[test]
fn test_log_simple() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "hello world"]);
    let out = repo.gg_ok(&["--simple", "log"]);
    assert!(out.contains("hello world"));
    assert!(out.contains("@"));
    // Simple mode should not contain Unicode graph chars
    assert!(!out.contains("◆"));
}

#[test]
fn test_log_n_flag() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    repo.write_file("f.txt", "b");
    repo.gg_ok(&["commit", "-m", "second"]);
    let out = repo.gg_ok(&["log", "-n", "1"]);
    assert!(out.contains("second"));
    assert!(!out.contains("first"));
}

// ── Diff ──

#[test]
fn test_diff_shows_changes() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "original\n");
    repo.gg_ok(&["commit", "-m", "initial"]);
    repo.write_file("f.txt", "original\nmodified\n");
    let out = repo.gg_ok(&["diff"]);
    assert!(out.contains("+modified"));
}

#[test]
fn test_diff_specific_path() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("a.txt", "a\n");
    repo.write_file("b.txt", "b\n");
    repo.gg_ok(&["commit", "-m", "initial"]);
    repo.write_file("a.txt", "a\nchanged\n");
    repo.write_file("b.txt", "b\nchanged\n");
    let out = repo.gg_ok(&["diff", "a.txt"]);
    assert!(out.contains("a.txt"));
    assert!(!out.contains("b.txt"));
}

#[test]
fn test_diff_no_changes() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "initial"]);
    let out = repo.gg_ok(&["diff"]);
    assert!(out.contains("No changes"));
}

// ── Describe ──

#[test]
fn test_describe_amends_message() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "old message"]);
    let out = repo.gg_ok(&["describe", "-m", "new message"]);
    assert!(out.contains("Updated commit"));
    let log = repo.gg_ok(&["log"]);
    assert!(log.contains("new message"));
    assert!(!log.contains("old message"));
}

#[test]
fn test_desc_alias() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "old"]);
    let out = repo.gg_ok(&["desc", "-m", "updated"]);
    assert!(out.contains("Updated commit"));
}

// ── New ──

#[test]
fn test_new_creates_empty_change() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    let out = repo.gg_ok(&["new"]);
    assert!(out.contains("Created new change"));
    let status = repo.gg_ok(&["status"]);
    assert!(status.contains("clean"));
}

#[test]
fn test_new_with_message() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    repo.gg_ok(&["new", "-m", "wip feature"]);
    let log = repo.gg_ok(&["log", "-n", "1"]);
    assert!(log.contains("wip feature"));
}

// ── Branch ──

#[test]
fn test_branch_create_and_list() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    repo.gg_ok(&["branch", "create", "feature"]);
    let out = repo.gg_ok(&["branch", "list"]);
    assert!(out.contains("feature"));
    assert!(out.contains("master"));
}

#[test]
fn test_branch_delete() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    repo.gg_ok(&["branch", "create", "feature"]);
    let out = repo.gg_ok(&["branch", "delete", "feature"]);
    assert!(out.contains("Deleted"));
    let list = repo.gg_ok(&["branch", "list"]);
    assert!(!list.contains("feature"));
}

#[test]
fn test_branch_set() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first"]);
    repo.gg_ok(&["branch", "create", "feature"]);
    repo.write_file("f.txt", "b");
    repo.gg_ok(&["commit", "-m", "second"]);
    let out = repo.gg_ok(&["branch", "set", "feature"]);
    assert!(out.contains("Set branch feature"));
}

// ── Error handling ──

#[test]
fn test_error_outside_repo() {
    let dir = tempfile::tempdir().unwrap();
    let bin = env!("CARGO_BIN_EXE_gg");
    let output = Command::new(bin)
        .args(["status"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not a gg/git repository"));
}

// ── Passthrough ──

#[test]
fn test_passthrough_to_git() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    // `gg version` should passthrough to `git version`
    let out = repo.gg_ok(&["version"]);
    assert!(out.contains("git version"));
}

// ── Default view (bare `gg`) ──

#[test]
fn test_bare_gg_shows_log_and_status() {
    let repo = TestRepo::new();
    repo.gg_ok(&["init"]);
    repo.write_file("f.txt", "a");
    repo.gg_ok(&["commit", "-m", "first commit"]);
    repo.write_file("g.txt", "b");
    let out = repo.gg_ok(&[]);
    // Should contain log output
    assert!(out.contains("first commit"));
    // Should contain status output (untracked file)
    assert!(out.contains("g.txt"));
}

#[test]
fn test_bare_gg_outside_repo() {
    let dir = tempfile::tempdir().unwrap();
    let bin = env!("CARGO_BIN_EXE_gg");
    let output = Command::new(bin)
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Not in a git repository"));
}
