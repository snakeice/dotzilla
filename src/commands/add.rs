use std::fs;

use anyhow::{Context, Result, anyhow};
use colored::*;

use crate::models::{Config, DotPath, DotfileEntry, DotfileStatus};

pub fn add_dotfile(mut config: Config, dotfile_path: DotPath) -> Result<()> {
    if !dotfile_path.abs_path.exists() {
        return Err(anyhow!(
            "Dotfile at {} does not exist",
            dotfile_path.abs_path.display()
        ));
    }

    if let Some(parent) = dotfile_path.abs_target.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create directory for target path: {}",
                    parent.display()
                )
            })?;
        }
    }

    if dotfile_path.abs_path.is_dir() {
        let mut opts = fs_extra::dir::CopyOptions::new();
        opts.copy_inside = true;
        opts.overwrite = true;
        opts.skip_exist = false;

        fs_extra::dir::copy(&dotfile_path.abs_path, &dotfile_path.abs_target, &opts)
            .with_context(|| {
                format!(
                    "Failed to copy directory from {} to {}",
                    dotfile_path.abs_path.display(),
                    dotfile_path.target.display()
                )
            })?;
    } else {
        fs::copy(&dotfile_path.abs_path, &dotfile_path.abs_target).with_context(|| {
            format!(
                "Failed to copy file from {} to {}",
                dotfile_path.abs_path.display(),
                dotfile_path.abs_target.display()
            )
        })?;
    }

    let mut entry = DotfileEntry::from_dotpath(&dotfile_path);
    entry.status = DotfileStatus::Tracked;

    config.add(&dotfile_path, entry)?;
    config.save()?;

    println!("{} Added dotfile: {}", "✓".green(), dotfile_path);
    Ok(())
}
