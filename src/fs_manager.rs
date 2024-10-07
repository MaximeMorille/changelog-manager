use std::{fs::File, io::prelude::*};

use crate::entry::{Entry, Serializable};

const UNRELEASED_CHANGELOGS_FOLDER: &str = "unreleased_changelogs";

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

pub fn read_entries() -> Vec<Entry> {
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
        entries.push(Entry::from_json(&content));
    }

    entries
}

pub fn write_changelog(content: String, changelog: &Option<String>) {
    // panic!("Not implemented yet");
    let mut file = File::create_new("CHANGELOG.md").expect("Unable to create file");

    file.write_all(content.as_bytes())
        .expect("Unable to write data");

    if let Some(changelog) = changelog {
        let mut file = File::create_new(changelog).expect("Unable to create file");

        file.write_all(content.as_bytes())
            .expect("Unable to write data");
    }
}
