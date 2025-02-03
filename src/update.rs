use reqwest::{header::USER_AGENT, Error};
use serde::Deserialize;

const LATEST_RELEASE_URL: &str =
    "http://api.github.com/repos/MaximeMorille/changelog-manager/releases/latest";

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    html_url: String,
}

pub fn check_for_updates() -> Result<(), Error> {
    let latest_release = get_latest_release()?;
    let current_version = env!("CARGO_PKG_VERSION");

    if is_newer_release(&latest_release, current_version) {
        println!(
            "A new version of changelog-manager is available: {}",
            latest_release.tag_name
        );
        println!("You can download it from: {}", latest_release.html_url);
    }

    Ok(())
}

fn is_newer_release(release: &Release, current_version: &str) -> bool {
    let latest_version = release.tag_name.to_string();

    return current_version
        .split('.')
        .zip(latest_version.split('.'))
        .any(|(a, b)| {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            a < b
        });
}

fn get_latest_release() -> Result<Release, Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(LATEST_RELEASE_URL)
        .header(USER_AGENT, "changelog-manager-client")
        .send()?;

    match response.error_for_status() {
        Ok(r) => {
            let release = r.json::<Release>()?;
            return Ok(release);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_newer_release() {
        let release = Release {
            tag_name: "0.1.0".to_string(),
            html_url: "plop".to_string(),
        };

        assert_eq!(is_newer_release(&release, "0.0.1"), true);
        assert_eq!(is_newer_release(&release, "0.1.0"), false);
        assert_eq!(is_newer_release(&release, "0.1.1"), false);
    }
}
