use std::process::Command;

pub struct GitInfo {
    branch: String,
    username: String,
}

pub trait GitInfoProvider {
    fn new() -> Self;
    fn get_branch(&self) -> String;
    fn get_username(&self) -> String;
}

impl GitInfoProvider for GitInfo {
    fn new() -> Self {
        GitInfo {
            username: execute_git_command(["config", "--get", "user.name"], "Failed to get current git user"),
            branch: execute_git_command(["rev-parse", "--abbrev-ref", "HEAD"], "Failed to get current git branch"),
        }
    }

    fn get_branch(&self) -> String {
        self.branch.clone()
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }
}

fn execute_git_command(git_args: [&str; 3], error_message: &str) -> String {
    let output = Command::new("git")
        .args(git_args)
        .output()
        .expect(error_message);

    let result = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        String::from("Unknown")
    };

    result
}