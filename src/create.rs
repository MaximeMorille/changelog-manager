use std::str::FromStr;

use json_writer::{JSONObjectWriter, JSONWriterValue, PrettyJSONWriter};
use slug::slugify;

use crate::{fs_manager::write_entry, git_info, EntryFields};

pub enum EntryType {
    Added,
    Changed,
    Fixed,
    Removed,
    Deprecated,
    Security,
    Technical,
}

impl FromStr for EntryType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADDED" => Ok(EntryType::Added),
            "CHANGED" => Ok(EntryType::Changed),
            "FIXED" => Ok(EntryType::Fixed),
            "REMOVED" => Ok(EntryType::Removed),
            "DEPRECATED" => Ok(EntryType::Deprecated),
            "SECURITY" => Ok(EntryType::Security),
            "TECHNICAL" => Ok(EntryType::Technical),
            _ => Err(()),
        }
    }
}

struct Entry {
    author: String,
    title: String,
    description: Option<String>,
    entry_type: EntryType,
    is_breaking_change: Option<bool>,
    issue: u32,
}

impl JSONWriterValue for Entry {
    fn write_json<W: json_writer::JSONWriter>(self, writer: &mut W) {
        let mut obj = JSONObjectWriter::new(writer);

        obj.value("author", &self.author);
        obj.value("title", &self.title);
        obj.value("description", &self.description.unwrap_or("".to_string()));
        obj.value(
            "type",
            match self.entry_type {
                EntryType::Added => "Added",
                EntryType::Changed => "Changed",
                EntryType::Fixed => "Fixed",
                EntryType::Removed => "Removed",
                EntryType::Deprecated => "Deprecated",
                EntryType::Security => "Security",
                EntryType::Technical => "Technical",
            },
        );
        obj.value("isBreakingChange", self.is_breaking_change.unwrap_or(false));
        obj.value("issue", self.issue);
        obj.end();
    }
}

// on veut créer une entrée à partir des options passées en entrée
pub fn start_interactive_mode() {
    panic!("Not implemented yet");
}

pub fn process_static_input<I: git_info::GitInfoProvider>(fields: &EntryFields, info: I) {
    let entry_type = EntryType::from_str(
        fields
            .entry_type
            .as_ref()
            .expect("entry_type is a mandatory argument")
            .as_str(),
    )
    .expect("Invalid entry type");

    // call git to get the current user
    let default_user = info.get_username();

    let entry = Entry {
        author: fields.author.as_ref().unwrap_or(&default_user).to_string(),
        title: fields
            .title
            .as_ref()
            .expect("title is mandatory")
            .to_string(),
        description: None,
        entry_type,
        is_breaking_change: fields.is_breaking_change,
        issue: *fields.issue.as_ref().expect("issue is mandatory"),
    };
    create_changelog_entry(entry, info.get_branch())
}

fn create_changelog_entry(entry: Entry, branch: String) {
    let filename = slugify(&branch);
    let buffer = to_pretty_json_string(entry);

    write_entry(filename, buffer);
}

fn to_pretty_json_string(entry: Entry) -> String {
    let mut buffer = String::new();
    entry.write_json(&mut PrettyJSONWriter::with_indent(&mut buffer, "    "));
    buffer
}

#[test]
fn test_minimalist_entry_to_json() {
    let entry = Entry {
        author: "Maxime Morille".to_string(),
        title: "Test".to_string(),
        entry_type: EntryType::Added,
        issue: 123,
        description: None,
        is_breaking_change: None,
    };
    assert_eq!(
        to_pretty_json_string(entry),
        r#"{
    "author": "Maxime Morille",
    "title": "Test",
    "description": "",
    "type": "Added",
    "isBreakingChange": false,
    "issue": 123
}"#
    );
}

#[test]
fn test_complete_entry_to_json() {
    let entry = Entry {
        author: "Maxime Morille".to_string(),
        title: "Test".to_string(),
        description: Some("This is a test".to_string()),
        entry_type: EntryType::Added,
        is_breaking_change: Some(true),
        issue: 123,
    };
    assert_eq!(
        to_pretty_json_string(entry),
        r#"{
    "author": "Maxime Morille",
    "title": "Test",
    "description": "This is a test",
    "type": "Added",
    "isBreakingChange": true,
    "issue": 123
}"#
    );
}
