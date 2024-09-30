use clap::{Args, Parser, Subcommand};
use git_info::{GitInfo, GitInfoProvider};

mod create;
mod fs_manager;
mod git_info;

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
        #[arg(short, long)]
        version: String,
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
                create::start_interactive_mode();
            } else {
                create::process_static_input(create_options, git_info);
            }
        }
        Some(Commands::Merge { version }) => {
            println!("Merging version {version}");
        }
        None => {}
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
