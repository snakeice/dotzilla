use anyhow::{Result, anyhow};
use colored::*;

use crate::{
    models::{Config, DotfileStatus},
    utils::path_compare::{self, CompareResult},
};

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
            DotfileStatus::Tracked => {
                let compare_result =
                    path_compare::compare_paths(&dotpath.abs_path, &dotpath.abs_target).map_err(
                        |err| anyhow!("Error comparing files: {}. Please check the paths.", err),
                    )?;

                match compare_result {
                    CompareResult::Equal => "[Tracked]".green(),
                    CompareResult::NotEqual => "[Diff Detected]".yellow(),
                    CompareResult::Linked => "[Linked]".blue(),
                }
            }
            DotfileStatus::Modified => "[Modified]".yellow(),
            _ => "[Unknown]".red(),
        };

        println!("{} ({})", status_str, dotpath);
    }

    println!();
    println!("{}", "Staged for linking:".bold());
    if config.get_staged().is_empty() {
        println!("No dotfiles staged. Use 'dotzilla stage <name>' to stage dotfiles.");
    } else {
        for dotpath in staged.keys() {
            let stage_status = if dotpath.target_staged.exists() {
                let compare_result =
                    path_compare::compare_paths(&dotpath.abs_path, &dotpath.abs_target_staged)
                        .map_err(|err| {
                            anyhow!("Error comparing files: {}. Please check the paths.", err)
                        })?;

                match compare_result {
                    CompareResult::Equal => "[In Staging]".green(),
                    CompareResult::NotEqual => "[Diff Detected]".yellow(),
                    CompareResult::Linked => "[Linked]".blue(),
                }
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
