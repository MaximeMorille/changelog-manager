use std::{io::Error, process::Command};

pub struct GitInfo {
    branch: String,
    username: String,
}

pub trait GitInfoProvider {
    fn new() -> Result<GitInfo, Error>;
    fn get_branch(&self) -> &String;
    fn get_username(&self) -> String;
}

impl GitInfoProvider for GitInfo {
    fn new() -> Result<GitInfo, Error> {
        Ok(GitInfo {
            username: execute_git_command(["config", "--get", "user.name"])?,
            branch: execute_git_command(["rev-parse", "--abbrev-ref", "HEAD"])?,
        })
    }

    fn get_branch(&self) -> &String {
        &self.branch
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }
}

fn execute_git_command(git_args: [&str; 3]) -> Result<String, Error> {
    let output = Command::new("git").args(git_args).output()?;

    let result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        String::from("Unknown")
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::git_info::execute_git_command;

    #[test]
    fn test_with_unknown_git_command() {
        let result = execute_git_command(["unknown", "command", "args"]).expect("Should not fail");
        assert_eq!(result, "Unknown");
    }
}
