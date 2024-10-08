use std::env;

use assert_fs::{
    prelude::{PathChild, PathCreateDir},
    TempDir,
};

pub fn setup_test_env() -> TempDir {
    let root = TempDir::new().unwrap();
    env::set_current_dir(&root).expect("Failed to setup root testing directory");

    let unreleased_changelogs = root.child("unreleased_changelogs");
    unreleased_changelogs
        .create_dir_all()
        .expect("Failed to create unreleased_changelogs directory");
    root
}
