use anyhow::{Result, anyhow};
use colored::*;

use crate::models::Config;

pub fn unstage_dotfile(config: &mut Config, name: &str) -> Result<()> {
    if config.staged.remove(name).is_some() {
        config.save()?;
        println!("{} Unstaged dotfile: {}", "✓".green(), name);
        Ok(())
    } else {
        Err(anyhow!("No such staged dotfile: {}", name))
    }
}
