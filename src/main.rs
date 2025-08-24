mod commands;
mod generator;
mod models;
mod tui;
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
        Some(Commands::Init { path }) => {
            let init_path = expand_tilde(&path);
            commands::init_repo(init_path)
        }
        Some(Commands::Add { path }) => {
            let config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &path);
            commands::add_dotfile(config, dot_path)
        }
        Some(Commands::Remove { name, keep }) => {
            let config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::remove_dotfile(config, dot_path, keep)
        }
        Some(Commands::Stage { name }) => {
            let mut config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::stage_dotfile(&mut config, &dot_path)
        }
        Some(Commands::Unstage { name }) => {
            let mut config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::unstage_dotfile(&mut config, &dot_path)
        }
        Some(Commands::Commit) => {
            let mut config = Config::load(&repo_path)?;
            commands::commit_dotfiles(&mut config)
        }
        Some(Commands::Link { name }) => {
            let config = Config::load(&repo_path)?;
            commands::link_dotfiles(&config, name)
        }
        Some(Commands::Unlink { name }) => {
            let config = Config::load(&repo_path)?;
            commands::unlink_dotfiles(&config, name)
        }
        Some(Commands::Status) => {
            let config = Config::load(&repo_path)?;
            commands::show_status(&config)
        }
        Some(Commands::List) => {
            let config = Config::load(&repo_path)?;
            commands::list_dotfiles(&config)
        }
        Some(Commands::Diff { name, tool, word }) => {
            let config = Config::load(&repo_path)?;
            let dot_path = DotPath::new(&config, &name);
            commands::show_diff(dot_path, tool, word)
        }
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            cmd.set_bin_name("dotzilla");
            if let Some(shell) = shell {
                print_completions(shell, &mut cmd);
            }
            Ok(())
        }
        Some(Commands::Tui) => tui::run(repo_path.to_string_lossy().to_string()),
        None => {
            // Default to TUI when no command is provided
            tui::run(repo_path.to_string_lossy().to_string())
        }
    }
}
