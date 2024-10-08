use std::fs;

use changelog_manager::{
    create,
    entry::{Builder, Entry, EntryType, Serializable},
};
use common::setup_test_env;
use pretty_assertions::assert_eq;

mod common;

fn assert_is_valid_json(filename: &str, expected_entry: &Entry) {
    let entry = fs::read_to_string(filename)
        .map(|content| Entry::from_json(&content))
        .expect("Error while reading file");

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

    let branch = "test_create".to_string();
    let entry = Entry::builder()
        .author("username".to_string())
        .title("Some title".to_string())
        .description(Some("A random description".to_string()))
        .r#type(EntryType::Added)
        .is_breaking_change(Some(false))
        .issue("42".to_string())
        .build();
    create::create_changelog_entry(&entry, &branch);

    assert!(
        fs::exists("./unreleased_changelogs/test-create.json")
            .expect("Error while checking if test-create.json exists"),
        "test-create.json should exist"
    );

    let expected_entry = Entry::builder()
        .author("username".to_string())
        .title("Some title".to_string())
        .description(Some("A random description".to_string()))
        .r#type(EntryType::Added)
        .issue("42".to_string())
        .build();
    assert_is_valid_json("./unreleased_changelogs/test-create.json", &expected_entry);
    drop(temp_dir);
}
