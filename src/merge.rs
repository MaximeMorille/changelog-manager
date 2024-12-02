use std::{collections::BTreeMap, error::Error};

use chrono::{DateTime, Local};

use crate::{
    entry::{Entry, Serializable},
    fs_manager,
};

pub fn merge_entries(
    version: &String,
    date: &Option<DateTime<Local>>,
    changelog: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    let entries = read_entries()?;
    let new_content = entries_to_string(entries, version, date)?;
    fs_manager::write_changelog(new_content, changelog)?;
    Ok(fs_manager::clear_entries()?)
}

fn read_entries() -> Result<Vec<Entry>, Box<dyn Error>> {
    let json_entries = fs_manager::read_entries()?;
    let entries: Result<Vec<Entry>, serde_json::Error> =
        json_entries.iter().map(Entry::from_json).collect();
    Ok(entries?)
}

fn entries_to_string(
    entries: Vec<Entry>,
    version: &String,
    date: &Option<DateTime<Local>>,
) -> Result<String, Box<dyn Error>> {
    if entries.is_empty() {
        return Ok(String::new());
    }

    let mut entry_map = BTreeMap::new();

    entries.iter().for_each(|entry| {
        let key = &entry.r#type;
        let value = entry;

        if let std::collections::btree_map::Entry::Vacant(e) = entry_map.entry(key) {
            e.insert(vec![value]);
        } else {
            entry_map.get_mut(&key).unwrap().push(value);
        }
    });

    let mut content = String::new();
    content.push_str(&format!(
        "## [{}] - {}\n",
        version,
        date.unwrap_or(Local::now()).format("%Y-%m-%d")
    ));

    let mut release_notes = String::new();
    entry_map.iter_mut().for_each(|(key, value)| {
        release_notes.push_str(&format!("\n### {}\n\n", key));
        value.sort();
        value.iter().for_each(|entry| {
            release_notes.push_str(&entry.to_markdown());
        });
    });
    println!("{}", release_notes);

    content.push_str(&format!("\n{}\n", release_notes.trim()));
    Ok(content)
}

#[cfg(test)]
mod tests {
    use assert_fs::{
        prelude::{PathChild, PathCreateDir},
        TempDir,
    };
    use chrono::{Local, TimeZone};
    use pretty_assertions::assert_eq;

    use crate::{
        entry::{Builder, Entry, EntryType},
        merge::{entries_to_string, read_entries},
    };

    #[test]
    fn test_empty_entries_to_string() {
        assert_eq!(
            "",
            entries_to_string(vec![], &"1.0.0".to_string(), &None)
                .expect("Should parse entries to string")
        );
    }

    #[test]
    fn test_entries_to_string() {
        let entries = vec![
            Entry::builder()
                .author("username".to_string())
                .title("Some title".to_string())
                .issue("42".to_string())
                .r#type(EntryType::Added)
                .build(),
            Entry::builder()
                .author("username".to_string())
                .title("Another title".to_string())
                .issue("43".to_string())
                .r#type(EntryType::Added)
                .build(),
            Entry::builder()
                .author("username".to_string())
                .title("A final title".to_string())
                .issue("64".to_string())
                .r#type(EntryType::Removed)
                .is_breaking_change(Some(true))
                .description(Some("A random description".to_string()))
                .build(),
        ];

        let expected = "## [1.0.0] - 2021-08-01\n\n### Added\n\n- [Another title](43)\n- [Some title](42)\n\n### Removed\n\n- [**BREAKING CHANGE** A final title](64)\n  A random description\n";
        let date = Local.with_ymd_and_hms(2021, 8, 1, 0, 0, 0);
        assert_eq!(
            expected,
            entries_to_string(entries, &"1.0.0".to_string(), &date.single())
                .expect("Should parse entries to string")
        );
    }

    #[test]
    fn test_read_empty_entries() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        std::env::set_current_dir(&temp_dir).expect("Failed to set current directory");
        temp_dir
            .child("unreleased_changelogs")
            .create_dir_all()
            .expect("Failed to create unreleased_changelogs directory");
        let entries = read_entries().expect("entries should be read");
        assert!(entries.is_empty());
    }
}
