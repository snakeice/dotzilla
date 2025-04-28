use anyhow::{Context, Result, anyhow};
use colored::*;
use std::fs;

use crate::models::{Config, DotPath, DotfileStatus};

pub fn stage_dotfile(config: &mut Config, dotfile_path: &DotPath) -> Result<()> {
    let entry = config.get_dotfile(&dotfile_path)?.clone();
    let mut staged_entry = entry.clone();
    staged_entry.status = DotfileStatus::Staged;

    // Ensure the source file exists
    if !dotfile_path.abs_path.exists() {
        return Err(anyhow!(
            "Source file does not exist: {}",
            dotfile_path.abs_path.display()
        ));
    }

    if let Some(parent) = dotfile_path.target_staged.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
            println!(
                "{} Created staging directory: {}",
                "✓".green(),
                parent.display()
            );
        }
    } 

    println!(
        "{} Staging file: {} -> {}",
        "✓".green(),
        dotfile_path.abs_path.display(),
        dotfile_path.target_staged.display()
    );

    // Copy the file to the repo
    if dotfile_path.abs_path.is_dir() {
        let mut opts = fs_extra::dir::CopyOptions::new();
        opts.copy_inside = true;
        opts.overwrite = true;
        opts.skip_exist = false;

        fs_extra::dir::copy(&dotfile_path.abs_path, &dotfile_path.abs_target_staged, &opts).with_context(
            || {
                format!(
                    "Failed to copy directory from {} to {}",
                    dotfile_path.abs_path.display(),
                    dotfile_path.target_staged.display()
                )
            },
        )?;
    } else {
        fs::copy(&dotfile_path.abs_path, &dotfile_path.abs_target_staged).with_context(|| {
            format!(
                "Failed to copy file from {} to {}",
                dotfile_path.abs_path.display(),
                dotfile_path.target_staged.display()
            )
        })?;
    }

    // Update the staged collection and save config
    config.stage(&dotfile_path, staged_entry)?;
    config.save()?;

    println!(
        "{} Staged dotfile: {}",
        "✓".green(),
        dotfile_path.to_name().display()
    );
    Ok(())
}
