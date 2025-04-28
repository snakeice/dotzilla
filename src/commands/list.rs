use anyhow::Result;
use colored::*;

use crate::models::Config;

pub fn list_dotfiles(config: &Config) -> Result<()> {
    println!("{} Dotfiles List", "✦".cyan());
    println!("{}", "===============".cyan());

    if config.get().is_empty() {
        println!("No dotfiles tracked. Use 'dotzilla add <path>' to add dotfiles.");
        return Ok(());
    }

    for (dotfile_path, entry) in config.get() {
        let staged = if config.get_staged().contains_key(&dotfile_path) {
            "(staged)".blue()
        } else {
            "".normal()
        };

        println!("{} {} {}", "•".cyan(), dotfile_path, staged);
        println!("  Target: {}", entry.target.display());
        println!();
    }

    Ok(())
}
