use std::{fs, path::PathBuf};

use config::{Config, ConfigError, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::update::Release;

const USER_SETTINGS_DIR: &str = "./.cm";
const SETTINGS_FILE: &str = "settings.toml";
const UPDATER_FILE: &str = "updater.toml";
const LOCAL_SETTINGS_FILE: &str = "./cm-rc.toml";

pub trait WeeklyCheck {
    fn is_older_than_week(&self) -> bool;
}

impl Default for Updater {
    fn default() -> Self {
        Updater {
            last_check: None,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: None,
        }
    }
}

pub trait Persist {
    fn persist(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait Update<T> {
    fn update(&mut self, values: T) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Updater {
    last_check: Option<String>,
    current_version: String,
    latest_version: Option<String>,
}

impl WeeklyCheck for Updater {
    fn is_older_than_week(&self) -> bool {
        println!("last_check: {:?}", self.last_check);
        let last_check = match self.last_check {
            Some(ref s) => s,
            None => &"1970-01-01T00:00:00Z".to_string(),
        };

        let last_check_date = match chrono::DateTime::parse_from_rfc3339(last_check) {
            Ok(date) => date.with_timezone(&chrono::Utc),
            Err(_) => return false,
        };
        let now = chrono::Utc::now();

        (now - last_check_date).num_days() >= 7
    }
}

impl Persist for Updater {
    fn persist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let updater_settings = Settings {
            updater: self.clone(),
        };
        let content: String = toml::to_string(&updater_settings)?;
        let updater_file_path = updater_file_path();
        let parent_folder = updater_file_path
            .parent()
            .expect("Failed to get parent folder of the updater file");

        if !parent_folder.exists() {
            fs::create_dir_all(parent_folder).expect("Failed to create settings directory");
        }
        std::fs::write(updater_file_path, content)?;

        Ok(())
    }
}

impl Update<Result<Release, Box<dyn std::error::Error>>> for Updater {
    fn update(
        &mut self,
        values: Result<Release, Box<dyn std::error::Error>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match values {
            Ok(release) => {
                self.latest_version = Some(release.tag_name);
                self.last_check = Some(chrono::Utc::now().to_rfc3339());
            }
            Err(e) => {
                eprintln!("Failed to update settings: {}", e);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub updater: Updater,
}

fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("eu", "morille", "changelog-manager")
}

fn settings_file_path() -> PathBuf {
    match project_dirs() {
        Some(project_dirs) => PathBuf::from(project_dirs.config_dir()).join(SETTINGS_FILE),
        None => PathBuf::from(USER_SETTINGS_DIR).join(SETTINGS_FILE),
    }
}

fn updater_file_path() -> PathBuf {
    match project_dirs() {
        Some(project_dirs) => PathBuf::from(project_dirs.config_dir()).join(UPDATER_FILE),
        None => PathBuf::from(USER_SETTINGS_DIR).join(UPDATER_FILE),
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .set_default(
                "updater.current_version",
                env!("CARGO_PKG_VERSION").to_string(),
            )?
            .add_source(File::from(settings_file_path()).required(false))
            .add_source(File::from(updater_file_path()).required(false))
            .add_source(File::with_name(LOCAL_SETTINGS_FILE).required(false))
            .build()?;
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::{Settings, Update, Updater};
    use crate::update::Release;

    #[test]
    fn test_settings() {
        let settings = Settings::new().unwrap();
        assert_eq!(settings.updater.current_version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_updater() {
        let mut updater: Updater = Default::default();

        let release = Release {
            tag_name: "0.1.0".to_string(),
            html_url: "http://example.com".to_string(),
        };

        updater.update(Ok(release)).unwrap();
        assert_eq!(updater.latest_version, Some("0.1.0".to_string()));
    }
}
