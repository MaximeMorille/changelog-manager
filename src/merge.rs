use std::collections::BTreeMap;

use chrono::{DateTime, Local};

use crate::{entry::Entry, fs_manager};

// on veut juste combiner tous les fichiers en un, monter la version
pub fn merge_entries(version: &String, date: &Option<DateTime<Local>>, changelog: &Option<String>) {
    let entries = fs_manager::read_entries();
    let new_content = entries_to_string(entries, version, date);
    fs_manager::write_changelog(new_content, changelog)
}

fn entries_to_string(
    entries: Vec<Entry>,
    version: &String,
    date: &Option<DateTime<Local>>,
) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut entry_map = BTreeMap::new();

    entries.iter().for_each(|entry| {
        let key = &entry.r#type;
        let value = entry;

        if let std::collections::btree_map::Entry::Vacant(e) = entry_map.entry(key) {
            e.insert(vec![value]);
        } else {
            entry_map.get_mut(&key).expect("Key not found").push(value);
        }
    });

    let mut content = String::new();
    content.push_str(&format!(
        "## [{}] - {}\n",
        version,
        date.unwrap_or(Local::now()).format("%Y-%m-%d")
    ));

    entry_map.iter().for_each(|(key, value)| {
        content.push_str(&format!("\n### {}\n\n", key));
        value.iter().for_each(|entry| {
            content.push_str(&entry.to_markdown());
        });
    });

    content
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};
    use pretty_assertions::assert_eq;

    use crate::{
        entry::{Builder, Entry, EntryType},
        merge::entries_to_string,
    };

    #[test]
    fn test_empty_entries_to_string() {
        assert_eq!("", entries_to_string(vec![], &"1.0.0".to_string(), &None));
    }

    #[test]
    fn test_entries_to_string() {
        let entries = vec![
            Entry::builder()
                .author("username".to_string())
                .title("Some title".to_string())
                .issue(42)
                .r#type(EntryType::Added)
                .build(),
            Entry::builder()
                .author("username".to_string())
                .title("Another title".to_string())
                .issue(43)
                .r#type(EntryType::Added)
                .build(),
            Entry::builder()
                .author("username".to_string())
                .title("A final title".to_string())
                .issue(64)
                .r#type(EntryType::Removed)
                .is_breaking_change(Some(true))
                .description(Some("A random description".to_string()))
                .build(),
        ];

        let expected = "## [1.0.0] - 2021-08-01\n\n### Added\n\n- [Some title](42)\n- [Another title](43)\n\n### Removed\n\n- [**BREAKING CHANGE** A final title](64)\n  A random description\n";
        let date = Local.with_ymd_and_hms(2021, 8, 1, 0, 0, 0);
        assert_eq!(
            expected,
            entries_to_string(entries, &"1.0.0".to_string(), &date.single())
        );
    }
}
