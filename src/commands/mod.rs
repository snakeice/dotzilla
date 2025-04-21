use clap::{Parser, Subcommand};

mod add;
mod diff;
mod init;
mod link;
mod list;
mod stage;
mod status;
mod unstage;

pub use add::add_dotfile;
use clap_complete::Shell;
pub use diff::show_diff;
pub use init::init_repo;
pub use link::link_dotfiles;
pub use list::list_dotfiles;
pub use stage::stage_dotfile;
pub use status::show_status;
pub use unstage::unstage_dotfile;

#[derive(Parser, )]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the dotfiles repository
    #[arg(short, long, default_value = "~/dotfiles", env = "DOTZILLA_REPO")]
    pub repo: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new dotzilla repository
    Init {
        /// Path to initialize the dotfiles repository
        #[arg(default_value = "~/dotfiles", env = "DOTZILLA_REPO")]
        path: String,
    },

    /// Add a dotfile to tracking
    Add {
        /// Path to the dotfile
        path: String,
    },

    /// Stage a dotfile for linking
    Stage {
        /// Name of the dotfile to stage
        name: String,
    },

    /// Unstage a dotfile
    Unstage {
        /// Name of the dotfile to unstage
        name: String,
    },

    /// Link all staged dotfiles to their target locations
    Link,

    /// Show the status of tracked and staged dotfiles
    Status,

    /// List all tracked dotfiles
    List,

    /// Show differences between tracked and local dotfiles
    Diff {
        /// Name of the dotfile to show differences for
        name: String,

        /// Compare using a specific diff tool (optional)
        #[arg(short, long)]
        tool: Option<String>,

        /// Show a word-by-word diff instead of line-by-line
        #[arg(short, long)]
        word: bool,
    },

    Completion {
        /// Generate shell completion script
        shell: Option<Shell>,
    },
}