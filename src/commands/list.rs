use anyhow::Result;
use colored::*;

use crate::models::Config;

pub fn list_dotfiles(config: &Config) -> Result<()> {
    println!("{} Dotfiles List", "✦".cyan());
    println!("{}", "===============".cyan());

    if config.dotfiles.is_empty() {
        println!("No dotfiles tracked. Use 'dotzilla add <path>' to add dotfiles.");
        return Ok(());
    }

    for (name, entry) in &config.dotfiles {
        let staged = if config.staged.contains_key(name) {
            "(staged)".blue()
        } else {
            "".normal()
        };

        println!("{} {} {}", "•".cyan(), name, staged);
        println!("  Source: {}", entry.source.display());
        println!("  Target: {}", entry.target.display());
        println!();
    }

    Ok(())
}
