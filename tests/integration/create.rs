use assert_cmd::Command;
use std::fs;

use crate::common::setup_test_env;
use changelog_manager::entry::{Builder, Entry, EntryType, Serializable};
use pretty_assertions::assert_eq;

fn assert_is_valid_json(filename: &str, expected_entry: &Entry) {
    let json = fs::read_to_string(filename).expect("Should read JSON file");
    let entry = Entry::from_json(&json).expect("Should parse json to Entry");

    assert_eq!(entry, *expected_entry);
}

#[test]
fn test_create() {
    let temp_dir = setup_test_env();

    assert!(
        fs::exists("./unreleased_changelogs")
            .expect("Error while checking if unreleased_changelogs exists"),
        "unreleased_changelogs should exist"
    );

    Command::cargo_bin("changelog-manager")
        .expect("Failed to build binary")
        .arg("create")
        .arg("Some title")
        .arg("--author")
        .arg("username")
        .arg("--description")
        .arg("A random description")
        .arg("--type")
        .arg("added")
        .arg("--issue")
        .arg("42")
        .assert()
        .success();

    assert!(
        fs::exists("./unreleased_changelogs/test-branch.json")
            .expect("Error while checking if test-branch.json exists"),
        "test-branch.json should exist"
    );

    let expected_entry = Entry::builder()
        .author("username".to_string())
        .title("Some title".to_string())
        .description(Some("A random description".to_string()))
        .r#type(EntryType::Added)
        .issue("42".to_string())
        .build();
    assert_is_valid_json("./unreleased_changelogs/test-branch.json", &expected_entry);
    drop(temp_dir);
}
