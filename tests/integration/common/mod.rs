use std::env;

use assert_cmd::Command;
use assert_fs::{
    prelude::{FileWriteStr, PathChild, PathCreateDir},
    TempDir,
};
use changelog_manager::entry::Builder;

pub fn setup_test_env() -> TempDir {
    let root = TempDir::new().unwrap();
    env::set_current_dir(&root).expect("Failed to setup root testing directory");

    Command::new("git").args(["init"]).assert().success();

    Command::new("git")
        .args(["config", "--local", "user.email", "test.user@mail.com"])
        .assert()
        .success();

    Command::new("git")
        .args(["config", "--local", "user.name", "Test User"])
        .assert()
        .success();

    Command::new("git")
        .args(["checkout", "-b", "test_branch"])
        .assert()
        .success();

    root.child("README.md").write_str("# Test Project").unwrap();

    Command::new("git").args(["add", "."]).assert().success();

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .assert()
        .success();

    let unreleased_changelogs = root.child("unreleased_changelogs");
    unreleased_changelogs
        .create_dir_all()
        .expect("Failed to create unreleased_changelogs directory");
    root
}

pub fn add_entry(
    branch: &str,
    title: &str,
    description: Option<&str>,
    r#type: changelog_manager::entry::EntryType,
    is_breaking_change: Option<bool>,
    issue: &str,
) {
    let entry = changelog_manager::entry::Entry::builder()
        .author("username".to_string())
        .title(title.to_string())
        .description(description.map(|s| s.to_string()))
        .r#type(r#type)
        .is_breaking_change(is_breaking_change)
        .issue(issue.to_string())
        .build();
    changelog_manager::create::create_changelog_entry(&entry, &branch.to_string())
        .expect("entry should be created");
}
