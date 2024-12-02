/// This module provides functionality for managing file system operations.
///
/// It includes utilities for working with files, such as reading from and writing to files.
use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::Path,
};

const UNRELEASED_CHANGELOGS_FOLDER: &str = "unreleased_changelogs";
const DEFAULT_CHANGELOG_PATH: &str = "CHANGELOG.md";
const BASE_CHANGELOG_CONTENT: &str = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
"#;

pub fn write_entry(filename: String, buffer: String) -> io::Result<()> {
    check_folder_existence()?;
    File::create_new(format!(
        "{}/{}.json",
        UNRELEASED_CHANGELOGS_FOLDER, filename
    ))?
    .write_all(buffer.as_bytes())
}

fn check_folder_existence() -> io::Result<()> {
    if std::path::Path::new(UNRELEASED_CHANGELOGS_FOLDER).exists() {
        Ok(())
    } else {
        std::fs::create_dir(UNRELEASED_CHANGELOGS_FOLDER)
    }
}

pub fn read_entries() -> Result<Vec<String>, io::Error> {
    let mut entries = Vec::new();
    let paths = std::fs::read_dir(UNRELEASED_CHANGELOGS_FOLDER)?
        .map(|rd| rd.expect("This error cannot happen"))
        .map(|de| de.path())
        .filter(|p| p.extension() == Some("json".as_ref()))
        .collect::<Vec<_>>();

    for path in paths {
        let content = std::fs::read_to_string(path)?;
        entries.push(content);
    }

    Ok(entries)
}

pub fn clear_entries() -> io::Result<()> {
    let paths = std::fs::read_dir(UNRELEASED_CHANGELOGS_FOLDER)?
        .map(|rd| rd.expect("This error cannot happen"))
        .map(|de| de.path())
        .filter(|p| p.extension() == Some("json".as_ref()))
        .collect::<Vec<_>>();

    for path in paths {
        std::fs::remove_file(&path)?;
    }

    Ok(())
}

pub fn write_changelog(content: String, changelog: &Option<String>) -> io::Result<()> {
    let changelog_path = match changelog {
        Some(path) => path,
        None => &DEFAULT_CHANGELOG_PATH.to_string(),
    };

    check_changelog_existence(changelog_path)?;

    if content.is_empty() {
        return Ok(());
    }

    let new_content = fs::read_to_string(changelog_path)?.replace(
        "## [Unreleased]\n",
        &format!("## [Unreleased]\n\n{}\n", content),
    );
    std::fs::write(changelog_path, new_content)
}

fn check_changelog_existence(changelog_path: &String) -> io::Result<()> {
    if !Path::new(changelog_path).exists() {
        fs::create_dir_all(Path::new(changelog_path).parent().unwrap())?;
        fs::write(changelog_path, BASE_CHANGELOG_CONTENT)?;
    }

    Ok(())
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
        env::set_current_dir(&root).expect("current dir should be set to temp dir");
        root
    }

    #[test]
    fn test_write_entry() {
        let temp_dir = setup_test_dir();
        write_entry("test".to_string(), "test".to_string()).expect("entry should be written");

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
        let entries = read_entries().expect("entries should be read");
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
        write_entry("first".to_string(), first_entry.to_string()).expect("entry should be written");
        write_entry("second".to_string(), second_entry.to_string())
            .expect("entry should be written");

        let entries = read_entries().expect("entries should be read");
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

        super::write_changelog("".to_string(), &None).expect("Error while writing changelog");

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
        )
        .expect("Error while writing changelog");

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
        super::write_changelog("New content".to_string(), &None)
            .expect("error while updating changelog");

        let file_content =
            std::fs::read_to_string(&changelog_path).expect("Error while reading file");
        assert_eq!(file_content, expected_content);
        drop(temp_dir);
    }
}
