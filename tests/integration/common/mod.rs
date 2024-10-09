use std::env;

use assert_fs::{
    prelude::{PathChild, PathCreateDir},
    TempDir,
};
use changelog_manager::entry::Builder;

pub fn setup_test_env() -> TempDir {
    let root = TempDir::new().unwrap();
    env::set_current_dir(&root).expect("Failed to setup root testing directory");

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
    changelog_manager::create::create_changelog_entry(&entry, &branch.to_string());
}
