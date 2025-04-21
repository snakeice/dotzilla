use anyhow::Result;
use colored::*;

use crate::models::{Config, DotfileStatus};

pub fn show_status(config: &Config) -> Result<()> {
    println!("{} Dotfiles Status", "✦".cyan());
    println!("{}", "=================".cyan());

    if config.dotfiles.is_empty() && config.staged.is_empty() {
        println!("No dotfiles tracked. Use 'dotzilla add <path>' to add dotfiles.");
        return Ok(());
    }

    println!("{}", "Tracked dotfiles:".bold());
    for (name, entry) in &config.dotfiles {
        let status_str = match entry.status {
            DotfileStatus::Tracked => "[Tracked]".green(),
            DotfileStatus::Modified => "[Modified]".yellow(),
            _ => "[Unknown]".red(),
        };

        println!("{} {} ({})", status_str, name, entry.source.display());
    }

    println!("");
    println!("{}", "Staged for linking:".bold());
    if config.staged.is_empty() {
        println!("No dotfiles staged. Use 'dotzilla stage <name>' to stage dotfiles.");
    } else {
        for (name, entry) in &config.staged {
            println!(
                "{} {} ({})",
                "[Staged]".blue(),
                name,
                entry.source.display()
            );
        }
    }

    Ok(())
}
