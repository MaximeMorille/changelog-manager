use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Represents the type of an entry in the changelog.
///
/// The `EntryType` enum is used to categorize the type of changes made in the project.
/// It derives several traits to facilitate comparison, serialization, and debugging.
///
/// # Variants
///
/// - `Added`: Represents an addition of a new feature.
/// - `Changed`: Represents a change in existing functionality. This is the default variant.
/// - `Fixed`: Represents a bug fix.
/// - `Removed`: Represents the removal of a feature.
/// - `Deprecated`: Represents a deprecated feature.
/// - `Security`: Represents a security-related change.
/// - `Technical`: Represents a technical change that doesn't fit into the other categories.
#[derive(
    Default, Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Ord, PartialOrd, Clone, ValueEnum,
)]
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

/// Implements the `FromStr` trait for `EntryType`.
///
/// This allows for converting a string representation of an entry type into an `EntryType` enum.
///
/// # Errors
///
/// Returns `Err(())` if the string does not match any of the known entry types.
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

/// Implements the `Display` trait for `EntryType`.
///
/// This allows for converting an `EntryType` enum into its string representation.
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

/// Represents an entry in the changelog.
///
/// The `Entry` struct contains information about a specific change made in the project.
/// It includes details such as the author, title, description, type of change, whether it's a breaking change, and the associated issue.
///
/// # Fields
///
/// - `author`: The author of the change.
/// - `title`: The title of the change.
/// - `description`: An optional description of the change.
/// - `type`: The type of the change, represented by the `EntryType` enum.
/// - `is_breaking_change`: A boolean indicating if the change is a breaking change.
/// - `issue`: The associated issue for the change.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    author: String,
    title: String,
    description: Option<String>,
    pub r#type: EntryType,
    is_breaking_change: bool,
    issue: String,
}

/// Implements methods for the `Entry` struct.
impl Entry {
    /// Creates a new `EntryBuilder` instance.
    pub fn builder() -> EntryBuilder {
        EntryBuilder::default()
    }

    /// Converts the `Entry` instance to a markdown string representation.
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

/// Implements the `Ord` trait for `Entry`.
///
/// Entries are compared first by whether they are breaking changes, and then by their titles.
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .is_breaking_change
            .cmp(&self.is_breaking_change)
            .then_with(|| self.title.cmp(&other.title))
    }
}

/// Implements the `PartialOrd` trait for `Entry`.
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Builder for creating `Entry` instances.
///
/// The `EntryBuilder` struct provides a builder pattern for constructing `Entry` instances.
/// It allows for setting various fields before building the final `Entry` instance.
#[derive(Default)]
pub struct EntryBuilder {
    author: String,
    title: String,
    description: Option<String>,
    r#type: EntryType,
    is_breaking_change: Option<bool>,
    issue: String,
}

/// Trait for building `Entry` instances.
pub trait Builder {
    fn author(self, author: String) -> Self;
    fn title(self, title: String) -> Self;
    fn description(self, description: Option<String>) -> Self;
    fn r#type(self, entry_type: EntryType) -> Self;
    fn is_breaking_change(self, is_breaking_change: Option<bool>) -> Self;
    fn issue(self, issue: String) -> Self;
    fn build(self) -> Entry;
}

/// Trait for serializing and deserializing `Entry` instances.
pub trait Serializable {
    fn to_json(&self) -> Result<String, Box<dyn Error>>;
    fn from_json(json: &String) -> Result<Entry, serde_json::Error>;
}

/// Implements the `Serializable` trait for `Entry`.
impl Serializable for Entry {
    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let formatter = PrettyFormatter::with_indent(b"    ");
        let mut buffer = Vec::with_capacity(128);

        let mut writer = Serializer::with_formatter(&mut buffer, formatter);
        self.serialize(&mut writer)?;

        Ok(String::from_utf8(buffer)?)
    }

    fn from_json(_json: &String) -> Result<Entry, serde_json::Error> {
        serde_json::from_str(_json)
    }
}

/// Implements the `Builder` trait for `EntryBuilder`.
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

    fn issue(mut self, issue: String) -> Self {
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
            issue: "123".to_string(),
            description: None,
            is_breaking_change: false,
        };
        assert_eq!(
            entry.to_json().expect("Should serialize to JSON"),
            r#"{
    "author": "Maxime Morille",
    "title": "Test",
    "description": null,
    "type": "Added",
    "isBreakingChange": false,
    "issue": "123"
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
            issue: "123".to_string(),
        };
        assert_eq!(
            entry.to_json().expect("Should serialize to JSON"),
            r#"{
    "author": "Maxime Morille",
    "title": "Test",
    "description": "This is a test",
    "type": "Added",
    "isBreakingChange": true,
    "issue": "123"
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
            issue: "123".to_string(),
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
            issue: "123".to_string(),
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

    #[test]
    fn test_entry_compare_with_no_breaking_change() {
        let entry1 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming first in alphabetical order".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: false,
        };

        let entry2 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming second in alphabetical order".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: false,
        };

        assert_eq!(entry1.cmp(&entry2), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_entry_compare_with_one_breaking_change() {
        let entry1 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming first in alphabetical order".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: false,
        };

        let entry2 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming second in alphabetical order, with a breaking change, should be first".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: true,
        };

        assert_eq!(entry1.cmp(&entry2), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_entry_compare_with_two_breaking_change() {
        let entry1 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming first in alphabetical order".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: true,
        };

        let entry2 = Entry {
            author: "Maxime Morille".to_string(),
            title: "A title coming second in alphabetical order".to_string(),
            r#type: EntryType::Added,
            issue: "123".to_string(),
            description: None,
            is_breaking_change: true,
        };

        assert_eq!(entry1.cmp(&entry2), std::cmp::Ordering::Less);
    }
}
