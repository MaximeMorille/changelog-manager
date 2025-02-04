use regex::Regex;
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
    if !is_valid_semver_version(current_version) {
        return false;
    }
    if !is_valid_semver_version(&release.tag_name) {
        return false;
    }

    let latest_version = release.tag_name.to_string();

    return current_version
        .split('.')
        .zip(latest_version.split('.'))
        .any(|(a, b)| {
            a.parse::<u32>()
                .and_then(|r| {
                    b.parse::<u32>().map(|l| {
                        if r < l {
                            return true;
                        }
                        false
                    })
                })
                .unwrap_or(false)
        });
}

fn is_valid_semver_version(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(version)
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
            Ok(release)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_valid_semver_version() {
        assert_eq!(is_valid_semver_version("0.1.0-alpha"), false);
    }

    #[rstest::rstest]
    #[case("0.1.0", "0.0.1", true)]
    #[case("0.1.0", "0.1.0", false)]
    #[case("0.1.0", "0.1.1", false)]
    #[case("2.1.3", "1.7.4", true)]
    #[case("2.0.0-alpha", "1.7.4", false)]
    #[case("2.3.4", "2.3.5-alpha.1", false)]
    #[case("1.10.0", "1.10.1", false)]
    fn test_is_newer_release(
        #[case] release_tag: &str,
        #[case] current_version: &str,
        #[case] expected: bool,
    ) {
        let release = Release {
            tag_name: release_tag.to_string(),
            html_url: "plop".to_string(),
        };

        assert_eq!(is_newer_release(&release, current_version), expected);
    }
}
