mod commands;
mod generator;
mod models;
mod utils;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use commands::{Cli, Commands};
use generator::print_completions;
use models::{Config, DotPath};
use utils::expand_tilde;

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
            let dot_path = DotPath::new(&config, &path);
            commands::add_dotfile(config, dot_path)
        }
        Commands::Stage { name } => {
            let mut config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::stage_dotfile(&mut config, &dot_path)
        }
        Commands::Unstage { name } => {
            let mut config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::unstage_dotfile(&mut config, &dot_path)
        }
        Commands::Commit => {
            let mut config = Config::load(&repo_path)?;
            commands::commit_dotfiles(&mut config)
        }
        Commands::Link { name } => {
            let config = Config::load(&repo_path)?;
            commands::link_dotfiles(&config, name)
        }
        Commands::Unlink { name } => {
            let config = Config::load(&repo_path)?;
            commands::unlink_dotfiles(&config, name)
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
            let dot_path = DotPath::new(&config, &name);
            commands::show_diff(dot_path, tool, word)
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            cmd.set_bin_name("dotzilla");
            if let Some(shell) = shell {
                print_completions(shell, &mut cmd);
            }
            Ok(())
        }
    }
}
