use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

mod dotfile;
pub use dotfile::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub repo_path: PathBuf,
    pub dotfiles: HashMap<String, DotfileEntry>,
    pub staged: HashMap<String, DotfileEntry>,
}

impl Config {
    pub fn new(repo_path: PathBuf) -> Self {
        Config {
            repo_path,
            dotfiles: HashMap::new(),
            staged: HashMap::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = self.repo_path.join(".dotzilla.json");
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_str)
            .with_context(|| format!("Failed to write config to {}", config_path.display()))
    }

    pub fn load(repo_path: &Path) -> Result<Self> {
        let config_path = repo_path.join(".dotzilla.json");
        if !config_path.exists() {
            return Ok(Config::new(repo_path.to_path_buf()));
        }

        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {}", config_path.display()))?;
        let config: Config = serde_json::from_str(&config_str)
            .with_context(|| format!("Failed to parse config from {}", config_path.display()))?;
        Ok(config)
    }

    pub fn get_dotfile(&self, name: &str) -> Result<&DotfileEntry> {
        self.dotfiles
            .get(name)
            .ok_or_else(|| anyhow!("No such dotfile: {}", name))
    }
}
