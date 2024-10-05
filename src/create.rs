use slug::slugify;

use crate::{
    entry::{Entry, Serializable},
    fs_manager::write_entry,
    git_info::GitInfo,
};

// on veut créer une entrée à partir des options passées en entrée
pub fn start_interactive_mode(info: GitInfo) {
    panic!("Not implemented yet");
}

pub fn create_changelog_entry(entry: &Entry, branch: &String) {
    let filename = slugify(branch);
    write_entry(filename, entry.to_json());
}
