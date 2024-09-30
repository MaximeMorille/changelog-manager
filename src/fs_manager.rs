use std::{

    fs::File,
    io::prelude::*,
};

pub fn write_entry(filename: String, buffer: String) {
    check_folder_existence();
    let mut file = File::create_new(format!("unreleased_changelogs/{}.json", filename))
        .expect("Unable to create file");

    file.write_all(buffer.as_bytes())
        .expect("Unable to write data");
}

fn check_folder_existence() {
    let path = "unreleased_changelogs";
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir(path).expect("Unable to create folder");
    }
}