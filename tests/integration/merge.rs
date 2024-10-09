use std::fs;

use crate::common::{add_entry, setup_test_env};
use changelog_manager::{entry, merge};
use chrono::{Local, TimeZone};
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
    let release_date = Local.with_ymd_and_hms(2024, 02, 15, 11, 02, 00);
    merge::merge_entries(&"1.0.0".to_string(), &release_date.single(), &None);

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

- [Some title](42)
- [**BREAKING CHANGE** Some title](44)

"#,
        content
    );

    drop(temp_dir);
}
