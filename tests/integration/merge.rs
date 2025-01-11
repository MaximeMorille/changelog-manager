use std::fs;

use crate::common::{add_entry, setup_test_env};
use assert_cmd::Command;
use changelog_manager::entry;
use pretty_assertions::assert_eq;

#[test]
fn test_merge_entries() {
    let temp_dir = setup_test_env();
    add_entry(
        "test-branch",
        "Some title",
        None,
        entry::EntryType::Changed,
        Some(false),
        "42",
    );
    add_entry(
        "test-branch-2",
        "Some important change",
        Some("Here we can have a migration note"),
        entry::EntryType::Added,
        Some(false),
        "43",
    );
    add_entry(
        "test-branch-3",
        "Some title",
        None,
        entry::EntryType::Changed,
        Some(true),
        "44",
    );

    Command::cargo_bin("changelog-manager")
        .expect("Failed to build binary")
        .arg("merge")
        .arg("1.0.0")
        .arg("--date")
        .arg("2024-02-15T11:02:00Z")
        .assert()
        .success();

    let content = fs::read_to_string("./CHANGELOG.md").expect("Error while reading CHANGELOG.md");
    assert_eq!(
        r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2024-02-15

### Added

- [Some important change](43)
  Here we can have a migration note

### Changed

- [**BREAKING CHANGE** Some title](44)
- [Some title](42)

"#,
        content
    );

    drop(temp_dir);
}
