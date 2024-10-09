use std::{
    fs::File,
    io::prelude::*,
};

const UNRELEASED_CHANGELOGS_FOLDER: &str = "unreleased_changelogs";
const DEFAULT_CHANGELOG_PATH: &str = "CHANGELOG.md";
const BASE_CHANGELOG_CONTENT: &str = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
"#;

pub fn write_entry(filename: String, buffer: String) {
    check_folder_existence();
    let mut file = File::create_new(format!(
        "{}/{}.json",
        UNRELEASED_CHANGELOGS_FOLDER, filename
    ))
    .expect("Unable to create file");

    file.write_all(buffer.as_bytes())
        .expect("Unable to write data");
}

fn check_folder_existence() {
    if !std::path::Path::new(UNRELEASED_CHANGELOGS_FOLDER).exists() {
        std::fs::create_dir(UNRELEASED_CHANGELOGS_FOLDER).expect("Unable to create folder");
    }
}

pub fn read_entries() -> Vec<String> {
    let mut entries = Vec::new();
    let paths = std::fs::read_dir(UNRELEASED_CHANGELOGS_FOLDER)
        .expect("Unable to read directory")
        .map(|res| res.map(|e| e.path()))
        .filter(|p| {
            p.as_ref()
                .is_ok_and(|p| p.extension() == Some("json".as_ref()))
        })
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("Error while collecting paths");

    for path in paths {
        let content = std::fs::read_to_string(path).expect("Error while reading file");
        entries.push(content);
    }

    entries
}

pub fn write_changelog(content: String, changelog: &Option<String>) {
    let changelog_path = match changelog {
        Some(path) => path,
        None => &DEFAULT_CHANGELOG_PATH.to_string(),
    };

    check_changelog_existence(changelog_path);

    if content.is_empty() {
        return;
    }

    let current_content =
        std::fs::read_to_string(changelog_path).expect("Error while reading file");
    let new_content = current_content.replace(
        "## [Unreleased]\n",
        &format!("## [Unreleased]\n\n{}\n", content),
    );
    std::fs::write(changelog_path, new_content).expect("Error while writing file");
}

fn check_changelog_existence(changelog_path: &String) {
    if !std::path::Path::new(changelog_path).exists() {
        std::fs::create_dir_all(std::path::Path::new(changelog_path).parent().unwrap())
            .expect("Unable to create folder");
        std::fs::write(changelog_path, BASE_CHANGELOG_CONTENT).expect("Unable to create folder");
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use assert_fs::{
        prelude::{PathChild, PathCreateDir},
        TempDir,
    };
    use pretty_assertions::assert_eq;

    use crate::fs_manager::{read_entries, write_entry};

    fn setup_test_dir() -> TempDir {
        let root = TempDir::new().unwrap();
        env::set_current_dir(&root).expect("Failed to setup root testing directory");
        root
    }

    #[test]
    fn test_write_entry() {
        let temp_dir = setup_test_dir();
        write_entry("test".to_string(), "test".to_string());

        assert!(std::path::Path::new("unreleased_changelogs/test.json").exists());
        drop(temp_dir);
    }

    #[test]
    fn test_read_empty_entries() {
        let temp_dir = setup_test_dir();
        temp_dir
            .child("unreleased_changelogs")
            .create_dir_all()
            .expect("Failed to create unreleased_changelogs directory");
        let entries = read_entries();
        assert!(entries.is_empty());
        drop(temp_dir);
    }

    #[test]
    fn test_read_entries() {
        let temp_dir = setup_test_dir();
        let first_entry = r#"{
    "author": "username",
    "title": "Some title",
    "description": "A random description",
    "type": "Added",
    "isBreakingChange": true,
    "issue": "https://gitlab.url/issues/42"
}"#;
        let second_entry = r#"{
    "author": "username",
    "title": "Another title",
    "type": "Changed",
    "isBreakingChange": false,
    "issue": "https://gitlab.url/issues/43"
}"#;
        write_entry("first".to_string(), first_entry.to_string());
        write_entry("second".to_string(), second_entry.to_string());

        let entries = read_entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], first_entry);
        drop(temp_dir);
    }

    #[test]
    fn test_write_changelog() {
        let temp_dir = setup_test_dir();
        let changelog_path = "CHANGELOG.md".to_string();
        let expected_content = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
"#;

        super::write_changelog("".to_string(), &None);

        let file_content =
            std::fs::read_to_string(&changelog_path).expect("Error while reading file");
        assert_eq!(file_content, expected_content);
        drop(temp_dir);
    }

    #[test]
    fn test_write_changelog_with_specific_path() {
        let temp_dir = setup_test_dir();
        let changelog_path = "./subfolder/CHANGELOG.md".to_string();
        let expected_content = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

New content
"#;

        super::write_changelog(
            "New content".to_string(),
            &Some("./subfolder/CHANGELOG.md".to_string()),
        );

        let file_content =
            std::fs::read_to_string(&changelog_path).expect("Error while reading file");
        assert_eq!(file_content, expected_content);
        drop(temp_dir);
    }

    #[test]
    fn test_update_changelog() {
        let temp_dir = setup_test_dir();
        let changelog_path = "CHANGELOG.md".to_string();
        let existing_content = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.3] - 2024-10-14

### Added

- Some new feature

"#;

        let expected_content = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

New content

## [1.2.3] - 2024-10-14

### Added

- Some new feature

"#;

        fs::write(&changelog_path, existing_content).expect("Error while writing file");
        super::write_changelog("New content".to_string(), &None);

        let file_content =
            std::fs::read_to_string(&changelog_path).expect("Error while reading file");
        assert_eq!(file_content, expected_content);
        drop(temp_dir);
    }
}
