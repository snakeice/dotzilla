use std::io;
use std::{fs, path::Path};

use anyhow::{Context, Result};
use colored::*;

use crate::models::Config;
use crate::utils::filter::filter_dotfiles_contains;

pub fn unlink_dotfiles(config: &Config, name: Option<String>) -> Result<()> {
    if config.get().is_empty() {
        println!("No dotfiles unlinking...");
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    let dotfiles = config.get();
    let filtered_dotfiles = filter_dotfiles_contains(dotfiles.iter(), name.as_deref());

    for (dotfile_path, _) in filtered_dotfiles {
        let source = &dotfile_path.abs_target;
        let target_path = &dotfile_path.abs_path;

        if target_path.exists() {
            if !target_path.is_symlink() {
                println!(
                    "{} Target path exists but is not a symlink: {}",
                    "✗".red(),
                    target_path.display()
                );
                error_count += 1;
                continue;
            }

            let target_link = fs::read_link(target_path)
                .with_context(|| format!("Failed to read symlink at {}", target_path.display()))?;

            let is_link_to_repo = target_link == *source;

            fs::remove_file(target_path).with_context(|| {
                format!(
                    "Failed to remove existing symlink at {}",
                    target_path.display()
                )
            })?;

            println!(
                "{} Removed symlink at {}",
                "✓".green(),
                target_path.display()
            );

            if is_link_to_repo && source.is_dir() {
                copy_dir_all(source, target_path).with_context(|| {
                    format!(
                        "Failed to copy directory from {} to {}",
                        source.display(),
                        target_path.display()
                    )
                })?;

                println!(
                    "{} Copied directory from {} to {}",
                    "✓".green(),
                    source.display(),
                    target_path.display()
                );
            } else if is_link_to_repo && source.is_file() {
                fs::copy(source, target_path).with_context(|| {
                    format!(
                        "Failed to copy file from {} to {}",
                        source.display(),
                        target_path.display()
                    )
                })?;

                println!(
                    "{} Copied file from {} to {}",
                    "✓".green(),
                    source.display(),
                    target_path.display()
                );
            }

            success_count += 1;
        } else if let Ok(metadata) = fs::symlink_metadata(target_path) {
            if metadata.file_type().is_symlink() && fs::metadata(target_path).is_err() {
                fs::remove_file(target_path).with_context(|| {
                    format!(
                        "Failed to remove broken symlink at {}",
                        target_path.display()
                    )
                })?;

                println!(
                    "{} Removed broken symlink at {}",
                    "✓".green(),
                    target_path.display()
                );

                success_count += 1;
            }
        } else {
            println!(
                "{} No symlink found at {}",
                "!".yellow(),
                target_path.display()
            );
        }
    }

    println!(
        "{} unlinked successfully, {} failed",
        success_count, error_count
    );
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.as_ref().join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(src_path, dst_path)?;
        } else if ty.is_file() {
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}
