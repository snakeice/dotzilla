use anyhow::{Result, anyhow};
use colored::*;
use std::fs;

use crate::models::{Config, DotfileStatus};

pub fn commit_dotfiles(config: &mut Config) -> Result<()> {
    if config.get().is_empty() {
        return Err(anyhow!(
            "No dotfiles staged for commit. Use 'dotzilla stage <name>' to stage dotfiles."
        ));
    }

    let mut commit_count = 0;

    for (dotpath, mut entry_staged) in std::mem::take(&mut config.get_staged()) {
        if let Some(parent) = dotpath.abs_target.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
                println!("{} Created directory: {}", "✓".green(), parent.display());
            }
        }

        if dotpath.target_staged.exists() {
            let mut bpk_path = dotpath.abs_target.clone();
            bpk_path.set_file_name(format!(
                "{}-dz-bpk",
                bpk_path.file_name().unwrap_or_default().to_string_lossy()
            ));

            if dotpath.abs_target.exists() {
                fs::rename(&dotpath.abs_target, &bpk_path).or_else(|err| {
                    Err(anyhow!(
                        "Failed to back up existing file: {}: {}",
                        dotpath.abs_target.display(),
                        err
                    ))
                })?;
            }

            fs::rename(&dotpath.abs_target_staged, &dotpath.abs_target)?;
            println!(
                "{} Copied file from staging to target: {} -> {}",
                "✓".green(),
                dotpath.target_staged.display(),
                dotpath.abs_target.display()
            );

            if bpk_path.exists() {
                if bpk_path.is_dir() {
                    fs::remove_dir_all(&bpk_path).or_else(|err| {
                        Err(anyhow!(
                            "Failed to remove backup directory: {}: {}",
                            bpk_path.display(),
                            err
                        ))
                    })?;
                } else {
                    fs::remove_file(&bpk_path).or_else(|err| {
                        Err(anyhow!(
                            "Failed to remove backup file: {}: {}",
                            bpk_path.display(),
                            err
                        ))
                    })?;
                }
                println!(
                    "{} Removed backup file: {}",
                    "✓".green(),
                    bpk_path.display()
                );
            }
        }

        config.unstage(&dotpath)?;

        entry_staged.status = DotfileStatus::Tracked;

        config.add(&dotpath, entry_staged)?;

        println!(
            "{} Committed dotfile: {}",
            "✓".green(),
            dotpath.to_name().display()
        );

        commit_count += 1;
    }

    if commit_count == 1 {
        println!("{} Committed 1 dotfile", "✓".green());
    } else {
        println!("{} Committed {} dotfiles", "✓".green(), commit_count);
    }

    Ok(())
}
