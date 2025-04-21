use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use colored::*;
use home::home_dir;

use crate::models::{Config, DotfileEntry, DotfileStatus};

pub fn add_dotfile(mut config: Config, dotfile_path: String) -> Result<()> {
    let path_str = dotfile_path.replace("~", home_dir().unwrap().to_str().unwrap());
    let source_path = PathBuf::from(&path_str);

    if !source_path.exists() {
        return Err(anyhow!(
            "Dotfile at {} does not exist",
            source_path.display()
        ));
    }

    let file_name = source_path.file_name().unwrap().to_str().unwrap();
    let target_path = config.repo_path.join(file_name);

    // Copy the file to the repo
    if source_path.is_dir() {
        fs_extra::dir::copy(
            &source_path,
            &config.repo_path,
            &fs_extra::dir::CopyOptions::new(),
        )
        .with_context(|| {
            format!(
                "Failed to copy directory from {} to {}",
                source_path.display(),
                target_path.display()
            )
        })?;
    } else {
        fs::copy(&source_path, &target_path).with_context(|| {
            format!(
                "Failed to copy file from {} to {}",
                source_path.display(),
                target_path.display()
            )
        })?;
    }

    // Add to tracked dotfiles
    let entry = DotfileEntry::new(source_path.clone(), target_path, DotfileStatus::Tracked);

    config.dotfiles.insert(file_name.to_string(), entry);
    config.save()?;

    println!("{} Added dotfile: {}", "✓".green(), file_name);
    Ok(())
}
