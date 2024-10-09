use std::str::FromStr;

use changelog_manager::{
    create,
    entry::{Builder, Entry, EntryType},
    git_info::{GitInfo, GitInfoProvider},
    merge,
};
use chrono::{DateTime, Local};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if there is a new version of this tool, and update it if needed
    Update {},
    /// Create a new Changelog entry
    Create {
        #[command(flatten)]
        create_options: EntryFields,
        /// Define the entry's content interactively
        #[arg(short, long)]
        interactive: bool,
    },
    /// Merge all entries in the CHANGELOG file
    Merge {
        /// Version of the new release to add to the CHANGELOG file
        #[arg(required = true)]
        version: String,
        /// Date of the new release (default: today)
        #[arg(short, long)]
        date: Option<DateTime<Local>>,
        /// Path to the CHANGELOG file (default: CHANGELOG.md)
        changelog: Option<String>,
    },
}

#[derive(Args)]
#[group(conflicts_with_all = ["interactive"])]
struct EntryFields {
    /// Author of the changes (default: current git user)
    #[arg(short, long)]
    author: Option<String>,
    /// Title of the change
    #[arg(required = true)]
    title: String,
    // Type of change
    #[arg(short, long, required = true)]
    r#type: String,
    /// Is this a breaking change? (default: false)
    #[arg(short = 'b', long)]
    is_breaking_change: Option<bool>,
    /// Issue URL
    #[arg(short = 'u', long, required = true)]
    issue: String,
    /// Description of the change
    #[arg(short, long)]
    description: Option<String>,
}

fn process_static_input<I: GitInfoProvider>(fields: &EntryFields, info: I) {
    let entry_type = EntryType::from_str(fields.r#type.as_str()).expect("Invalid entry type");

    // call git to get the current user
    let default_user = info.get_username();

    let entry = Entry::builder()
        .author(fields.author.as_ref().unwrap_or(&default_user).to_string())
        .title(fields.title.to_string())
        .r#type(entry_type)
        .is_breaking_change(fields.is_breaking_change)
        .issue(fields.issue.to_string())
        .description(fields.description.as_ref().map(|s| s.to_string()))
        .build();

    create::create_changelog_entry(&entry, info.get_branch())
}

fn main() {
    let cli = Cli::parse();
    let git_info = GitInfo::new();

    match &cli.command {
        Some(Commands::Update {}) => {
            println!("Updating tool");
        }
        Some(Commands::Create {
            create_options,
            interactive,
        }) => {
            if *interactive {
                create::start_interactive_mode(git_info);
            } else {
                process_static_input(create_options, git_info);
            }
        }
        Some(Commands::Merge {
            version,
            date,
            changelog,
        }) => {
            merge::merge_entries(version, date, changelog);
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
