use anyhow::Result;
use colored::*;

use crate::models::{Config, DotfileStatus};

pub fn show_status(config: &Config) -> Result<()> {
    println!("{} Dotfiles Status", "✦".cyan());
    println!("{}", "=================".cyan());

    let staged = config.get_staged();

    if config.get().is_empty() && staged.is_empty() {
        println!("No dotfiles tracked. Use 'dotzilla add <path>' to add dotfiles.");
        return Ok(());
    }

    println!("{}", "Tracked dotfiles:".bold());
    for (dotpath, entry) in config.get() {
        let status_str = match entry.status {
            DotfileStatus::Tracked => "[Tracked]".green(),
            DotfileStatus::Modified => "[Modified]".yellow(),
            _ => "[Unknown]".red(),
        };

        println!("{} ({})", status_str, dotpath);
    }

    println!("");
    println!("{}", "Staged for linking:".bold());
    if config.get_staged().is_empty() {
        println!("No dotfiles staged. Use 'dotzilla stage <name>' to stage dotfiles.");
    } else {
        for (dotpath, _) in &staged {
            let stage_status = if dotpath.target_staged.exists() {
                "[In Staging]".blue()
            } else {
                "[Pending]".yellow()
            };

            println!(
                "{} {} ({})",
                "[Staged]".blue(),
                stage_status,
                dotpath.to_name().display()
            );
        }
    }

    Ok(())
}
