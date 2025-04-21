use anyhow::Result;
use colored::*;

use crate::models::{Config, DotfileStatus};

pub fn stage_dotfile(config: &mut Config, name: &str) -> Result<()> {
    let entry = config.get_dotfile(name)?.clone();
    let mut staged_entry = entry.clone();
    staged_entry.set_status(DotfileStatus::Staged);

    config.staged.insert(name.to_string(), staged_entry);
    config.save()?;

    println!("{} Staged dotfile: {}", "✓".green(), name);
    Ok(())
}
