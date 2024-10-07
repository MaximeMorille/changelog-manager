use slug::slugify;

use crate::{
    entry::{Entry, Serializable},
    fs_manager::write_entry,
    git_info::GitInfo,
};

pub fn start_interactive_mode(info: GitInfo) {
    panic!("Not implemented yet");
}

pub fn create_changelog_entry(entry: &Entry, branch: &String) {
    let filename = slugify(branch);
    write_entry(filename, entry.to_json());
}

#[cfg(test)]
mod tests {
    use crate::{
        create::start_interactive_mode,
        git_info::{GitInfo, GitInfoProvider},
    };

    #[test]
    #[should_panic]
    fn test_start_interactive_mode() {
        start_interactive_mode(GitInfo::new());
    }
}
