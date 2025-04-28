use anyhow::{Result, anyhow};
use colored::*;
use std::fs;

use crate::models::{Config, DotPath};

pub fn unstage_dotfile(config: &mut Config, dotfile_path: &DotPath) -> Result<()> {
    if config.unstage(dotfile_path).is_ok() {
        if dotfile_path.target_staged.exists() {
            fs::remove_file(&dotfile_path.target_staged)?;
            println!(
                "{} Removed file from staging: {}",
                "✓".green(),
                dotfile_path.target_staged.display()
            );
        }

        config.save()?;
        println!(
            "{} Unstaged dotfile: {}",
            "✓".green(),
            dotfile_path.to_name().display()
        );
        Ok(())
    } else {
        Err(anyhow!(
            "No such staged dotfile: {}",
            dotfile_path.to_name().display()
        ))
    }
}
