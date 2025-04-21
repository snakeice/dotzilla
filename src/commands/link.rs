use std::fs;
use std::os::unix::fs as unix_fs;

use anyhow::{Context, Result};
use colored::*;
use log::error;

use crate::models::Config;

pub fn link_dotfiles(config: &Config) -> Result<()> {
    if config.staged.is_empty() {
        println!("No dotfiles staged for linking. Use 'dotzilla stage <name>' to stage dotfiles.");
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for (name, entry) in &config.staged {
        let source = &entry.target; // The file in the repo
        let target = &entry.source; // The original location

        if target.exists() {
            if target.is_symlink() {
                fs::remove_file(target).with_context(|| {
                    format!("Failed to remove existing symlink at {}", target.display())
                })?;
            } else if target.is_dir() {
                fs::remove_dir_all(target).with_context(|| {
                    format!(
                        "Failed to remove existing directory at {}",
                        target.display()
                    )
                })?;
            } else {
                fs::remove_file(target).with_context(|| {
                    format!("Failed to remove existing file at {}", target.display())
                })?;
            }
        }

        if let Some(parent) = target.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create parent directory at {}", parent.display())
                })?;
            }
        }

        match unix_fs::symlink(source, target) {
            Ok(_) => {
                println!("{} Linked: {} -> {}", "✓".green(), name, source.display());
                success_count += 1;
            }
            Err(e) => {
                error!(
                    "Failed to create symlink from {} to {}: {}",
                    source.display(),
                    target.display(),
                    e
                );
                println!("{} Failed to link {}: {}", "✗".red(), name, e);
                error_count += 1;
            }
        }
    }

    println!(
        "{} linked successfully, {} failed",
        success_count, error_count
    );
    Ok(())
}
