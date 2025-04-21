mod commands;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;
use commands::{Cli, Commands};
use models::Config;
use utils::path::expand_tilde;

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let repo_path = expand_tilde(&cli.repo);

    match cli.command {
        Commands::Init { path } => {
            let init_path = expand_tilde(&path);
            commands::init_repo(init_path)
        }
        Commands::Add { path } => {
            let config = Config::load(&repo_path)?;
            commands::add_dotfile(config, path)
        }
        Commands::Stage { name } => {
            let mut config = Config::load(&repo_path)?;
            commands::stage_dotfile(&mut config, &name)
        }
        Commands::Unstage { name } => {
            let mut config = Config::load(&repo_path)?;
            commands::unstage_dotfile(&mut config, &name)
        }
        Commands::Link => {
            let config = Config::load(&repo_path)?;
            commands::link_dotfiles(&config)
        }
        Commands::Status => {
            let config = Config::load(&repo_path)?;
            commands::show_status(&config)
        }
        Commands::List => {
            let config = Config::load(&repo_path)?;
            commands::list_dotfiles(&config)
        }
        Commands::Diff { name, tool, word } => {
            let config = Config::load(&repo_path)?;
            commands::show_diff(&config, &name, tool, word)
        }
    }
}
