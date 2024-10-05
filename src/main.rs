use std::str::FromStr;

use changelog_manager::{
    create::{create_changelog_entry, start_interactive_mode},
    entry::{Builder, Entry, EntryType},
    git_info::{GitInfo, GitInfoProvider},
};
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
        #[arg(short, long, required = true)]
        version: String,
        /// Date of the new release (default: today)
        #[arg(short, long)]
        date: String,
    },
}

#[derive(Args)]
#[group(conflicts_with_all = ["interactive"])]
struct EntryFields {
    /// Author of the changes (default: current git user)
    #[arg(short, long)]
    author: Option<String>,
    /// Title of the change
    #[arg(short, long, required = true)]
    title: Option<String>,
    // Type of change
    #[arg(short, long, required = true)]
    entry_type: Option<String>,
    /// Is this a breaking change? (default: false)
    #[arg(short = 'b', long)]
    is_breaking_change: Option<bool>,
    /// Issue number
    #[arg(short = 'n', long, required = true)]
    issue: Option<u32>,
}

fn process_static_input<I: GitInfoProvider>(fields: &EntryFields, info: I) {
    let entry_type = EntryType::from_str(
        fields
            .entry_type
            .as_ref()
            .expect("entry_type is a mandatory argument")
            .as_str(),
    )
    .expect("Invalid entry type");

    // call git to get the current user
    let default_user = info.get_username();

    let entry = Entry::builder()
        .author(fields.author.as_ref().unwrap_or(&default_user).to_string())
        .title(
            fields
                .title
                .as_ref()
                .expect("title is mandatory")
                .to_string(),
        )
        .entry_type(entry_type)
        .is_breaking_change(fields.is_breaking_change)
        .issue(*fields.issue.as_ref().expect("issue is mandatory"))
        .build();

    create_changelog_entry(&entry, info.get_branch())
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
                start_interactive_mode(git_info);
            } else {
                process_static_input(create_options, git_info);
            }
        }
        Some(Commands::Merge { version, date }) => {
            // merge::
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
