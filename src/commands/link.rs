use std::fs;
use std::os::unix::fs as unix_fs;

use anyhow::{Context, Result};
use colored::*;
use log::error;

use crate::models::Config;

pub fn link_dotfiles(config: &Config) -> Result<()> {
    if config.get().is_empty() {
        println!("No dotfiles staged for linking. Use 'dotzilla stage <n>' to stage dotfiles.");
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for (dotfile_path, _) in &config.get_staged() {
        let source = &dotfile_path.target;
        let target_path = &dotfile_path.abs_path;

        if target_path.exists() {
            if target_path.is_symlink() {
                fs::remove_file(&target_path).with_context(|| {
                    format!(
                        "Failed to remove existing symlink at {}",
                        target_path.display()
                    )
                })?;
            } else if target_path.is_dir() {
                fs::remove_dir_all(&target_path).with_context(|| {
                    format!(
                        "Failed to remove existing directory at {}",
                        target_path.display()
                    )
                })?;
            } else {
                fs::remove_file(&target_path).with_context(|| {
                    format!(
                        "Failed to remove existing file at {}",
                        target_path.display()
                    )
                })?;
            }
        }

        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create parent directory at {}", parent.display())
                })?;
            }
        }

        match unix_fs::symlink(source, &target_path) {
            Ok(_) => {
                println!(
                    "{} Linked: {} -> {}",
                    "✓".green(),
                    dotfile_path.to_name().display(),
                    source.display()
                );
                success_count += 1;
            }
            Err(e) => {
                error!(
                    "Failed to create symlink from {} to {}: {}",
                    source.display(),
                    target_path.display(),
                    e
                );
                println!(
                    "{} Failed to link {}: {}",
                    "✗".red(),
                    dotfile_path.to_name().display(),
                    e
                );
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
