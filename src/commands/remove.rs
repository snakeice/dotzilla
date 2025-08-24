use std::fs;

use anyhow::{Context, Result};
use colored::*;

use crate::models::{Config, DotPath};
use crate::utils;

pub fn remove_dotfile(mut config: Config, dotfile_path: DotPath, keep_file: bool) -> Result<()> {
    // Check if the dotfile is tracked
    let _entry = config.get_dotfile(&dotfile_path)?;

    // If keep_file is false, ask for confirmation to delete the file
    if !keep_file {
        let message = format!(
            "Are you sure you want to remove {} from tracking and delete the file at {}?",
            dotfile_path,
            dotfile_path.abs_target.display()
        );

        if !utils::confirm(&message, Some(false)) {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    // Remove from tracking
    config.remove(dotfile_path.clone())?;

    println!(
        "{} Removed dotfile from tracking: {}",
        "✓".green(),
        dotfile_path
    );

    // If keep_file is false, delete the actual file/directory
    if !keep_file {
        if dotfile_path.abs_target.exists() {
            if dotfile_path.abs_target.is_dir() {
                fs::remove_dir_all(&dotfile_path.abs_target).with_context(|| {
                    format!(
                        "Failed to remove directory at {}",
                        dotfile_path.abs_target.display()
                    )
                })?;
                println!(
                    "{} Removed directory: {}",
                    "✓".green(),
                    dotfile_path.abs_target.display()
                );
            } else {
                fs::remove_file(&dotfile_path.abs_target).with_context(|| {
                    format!(
                        "Failed to remove file at {}",
                        dotfile_path.abs_target.display()
                    )
                })?;
                println!(
                    "{} Removed file: {}",
                    "✓".green(),
                    dotfile_path.abs_target.display()
                );
            }
        } else {
            println!(
                "{} File/directory does not exist: {}",
                "!".yellow(),
                dotfile_path.abs_target.display()
            );
        }
    }

    Ok(())
}
