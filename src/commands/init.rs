use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::*;

use crate::models::Config;

pub fn init_repo(path: PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create directory at {}", path.display()))?;
        println!("{} Created directory at {}", "✓".green(), path.display());
    }

    if Config::load(&path).is_ok() {
        println!(
            "{} Dotzilla repository already exists at {}",
            "✗".red(),
            path.display()
        );
        return Ok(());
    }

    let config = Config::new(path.clone());
    config.save()?;

    println!(
        "{} Initialized dotzilla repository at {}",
        "✓".green(),
        path.display()
    );
    Ok(())
}
