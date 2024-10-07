use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Ord, PartialOrd)]
pub enum EntryType {
    Added,
    #[default]
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

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Added => write!(f, "Added"),
            EntryType::Changed => write!(f, "Changed"),
            EntryType::Fixed => write!(f, "Fixed"),
            EntryType::Removed => write!(f, "Removed"),
            EntryType::Deprecated => write!(f, "Deprecated"),
            EntryType::Security => write!(f, "Security"),
            EntryType::Technical => write!(f, "Technical"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    author: String,
    title: String,
    description: Option<String>,
    pub r#type: EntryType,
    is_breaking_change: bool,
    issue: u32,
}

impl Entry {
    pub fn builder() -> EntryBuilder {
        EntryBuilder::default()
    }

    pub fn to_markdown(&self) -> String {
        let prefix = match self.is_breaking_change {
            true => "**BREAKING CHANGE** ",
            false => "",
        };

        let description = match &self.description {
            Some(description) => format!("\n  {}", description),
            None => "".to_string(),
        };

        format!(
            "- [{prefix}{title}]({issue}){description}\n",
            prefix = prefix,
            title = self.title,
            issue = self.issue,
            description = description
        )
    }
}

#[derive(Default)]
pub struct EntryBuilder {
    author: String,
    title: String,
    description: Option<String>,
    r#type: EntryType,
    is_breaking_change: Option<bool>,
    issue: u32,
}

pub trait Builder {
    fn author(self, author: String) -> Self;
    fn title(self, title: String) -> Self;
    fn description(self, description: Option<String>) -> Self;
    fn r#type(self, entry_type: EntryType) -> Self;
    fn is_breaking_change(self, is_breaking_change: Option<bool>) -> Self;
    fn issue(self, issue: u32) -> Self;
    fn build(self) -> Entry;
}

pub trait Serializable {
    fn to_json(&self) -> String;
    fn from_json(json: &String) -> Self;
}

impl Serializable for Entry {
    fn to_json(&self) -> String {
        let formatter = PrettyFormatter::with_indent(b"    ");
        let mut buffer = Vec::with_capacity(128);

        let mut writer = Serializer::with_formatter(&mut buffer, formatter);
        self.serialize(&mut writer)
            .expect("Failed to serialize Entry");

        String::from_utf8(buffer).expect("Plop")
    }

    fn from_json(_json: &String) -> Self {
        serde_json::from_str(_json).expect("Failed to deserialize Entry")
    }
}

impl Builder for EntryBuilder {
    fn author(mut self, author: String) -> Self {
        self.author = author;
        self
    }

    fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    fn r#type(mut self, entry_type: EntryType) -> Self {
        self.r#type = entry_type;
        self
    }

    fn is_breaking_change(mut self, is_breaking_change: Option<bool>) -> Self {
        self.is_breaking_change = is_breaking_change;
        self
    }

    fn issue(mut self, issue: u32) -> Self {
        self.issue = issue;
        self
    }

    fn build(self) -> Entry {
        Entry {
            author: self.author,
            title: self.title,
            description: self.description,
            r#type: self.r#type,
            is_breaking_change: self.is_breaking_change.unwrap_or(false),
            issue: self.issue,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::entry::{Entry, EntryType, Serializable};

    #[test]
    fn test_minimalist_entry_to_json() {
        let entry = Entry {
            author: "Maxime Morille".to_string(),
            title: "Test".to_string(),
            r#type: EntryType::Added,
            issue: 123,
            description: None,
            is_breaking_change: false,
        };
        assert_eq!(
            entry.to_json(),
            r#"{
    "author": "Maxime Morille",
    "title": "Test",
    "description": null,
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
            r#type: EntryType::Added,
            is_breaking_change: true,
            issue: 123,
        };
        assert_eq!(
            entry.to_json(),
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

    #[test]
    fn test_complete_entry_to_markdown() {
        let entry = Entry {
            author: "Maxime Morille".to_string(),
            title: "Test".to_string(),
            description: Some("This is a test".to_string()),
            r#type: EntryType::Added,
            is_breaking_change: true,
            issue: 123,
        };

        assert_eq!(
            "- [**BREAKING CHANGE** Test](123)\n  This is a test\n",
            entry.to_markdown()
        );
    }

    #[test]
    fn test_simplest_entry_to_markdown() {
        let entry = Entry {
            author: "Maxime Morille".to_string(),
            title: "Test".to_string(),
            r#type: EntryType::Added,
            issue: 123,
            description: None,
            is_breaking_change: false,
        };

        assert_eq!("- [Test](123)\n", entry.to_markdown());
    }

    #[rstest::rstest]
    #[case(EntryType::Added, "Added")]
    #[case(EntryType::Changed, "Changed")]
    #[case(EntryType::Fixed, "Fixed")]
    #[case(EntryType::Removed, "Removed")]
    #[case(EntryType::Deprecated, "Deprecated")]
    #[case(EntryType::Security, "Security")]
    #[case(EntryType::Technical, "Technical")]
    fn test_entry_type_display(#[case] entry_type: EntryType, #[case] expected: &str) {
        assert_eq!(entry_type.to_string(), expected);
    }

    #[rstest::rstest]
    #[case("ADDED", EntryType::Added)]
    #[case("CHANGED", EntryType::Changed)]
    #[case("FIXED", EntryType::Fixed)]
    #[case("REMOVED", EntryType::Removed)]
    #[case("DEPRECATED", EntryType::Deprecated)]
    #[case("SECURITY", EntryType::Security)]
    #[case("TECHNICAL", EntryType::Technical)]
    fn test_entry_type_from_str(#[case] entry_type: &str, #[case] expected: EntryType) {
        assert_eq!(EntryType::from_str(entry_type).unwrap(), expected);
    }

    #[test]
    fn test_entry_type_from_str_invalid() {
        assert!(EntryType::from_str("INVALID").is_err());
    }
}
