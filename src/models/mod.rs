use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

mod dotfile;
pub use dotfile::*;

mod path;
pub use path::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    repo_path: PathBuf,
    #[serde(skip)]
    staging_path: PathBuf,
    dotfiles: HashMap<PathBuf, DotfileEntry>,
    staged: HashMap<PathBuf, DotfileEntry>,
}

impl Config {
    pub fn new(repo_path: PathBuf) -> Self {
        let staging_path = repo_path.join(".staging");

        Config {
            repo_path,
            staging_path,
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
            return Err(anyhow!(
                "No dotzilla repository found at {}, please run 'dotzilla init'",
                repo_path.display()
            ));
        }

        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {}", config_path.display()))?;
        let mut config: Config = serde_json::from_str(&config_str)
            .with_context(|| format!("Failed to parse config from {}", config_path.display()))?;

        config.repo_path = repo_path.to_path_buf();
        config.staging_path = repo_path.join(".staging");
        Ok(config)
    }

    pub fn get_dotfile(&self, dotpath: &DotPath) -> Result<&DotfileEntry> {
        let dot = self
            .dotfiles
            .get(dotpath.to_name())
            .ok_or_else(|| anyhow!("No such dotfile: {}", dotpath))?;

        Ok(dot)
    }

    pub fn add(&mut self, dotpath: &DotPath, entry: DotfileEntry) -> Result<()> {
        self.dotfiles.insert(dotpath.to_name().to_path_buf(), entry);
        self.save()
            .with_context(|| format!("Failed to save config after adding {}", dotpath))
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, dotpath: DotPath) -> Result<()> {
        if self.dotfiles.remove(dotpath.to_name()).is_none() {
            return Err(anyhow!("No such dotfile: {}", dotpath));
        }
        self.save()
            .with_context(|| format!("Failed to save config after removing {}", dotpath))
    }

    pub fn get(&self) -> HashMap<DotPath, DotfileEntry> {
        self.dotfiles
            .iter()
            .map(|(k, v)| (DotPath::from_path(self, k), v.clone()))
            .collect::<HashMap<DotPath, DotfileEntry>>()
    }

    #[allow(dead_code)]
    pub fn get_staged_dotfile(&self, dotpath: DotPath) -> Result<&DotfileEntry> {
        let dot = self
            .staged
            .get(dotpath.to_name())
            .ok_or_else(|| anyhow!("No such dotfile: {}", dotpath))?;

        Ok(dot)
    }

    pub fn stage(&mut self, dotpath: &DotPath, entry: DotfileEntry) -> Result<()> {
        self.staged.insert(dotpath.to_name().to_path_buf(), entry);
        self.save()
            .with_context(|| format!("Failed to save config after adding {}", dotpath))
    }

    pub fn unstage(&mut self, dotpath: &DotPath) -> Result<()> {
        if self.staged.remove(dotpath.to_name()).is_none() {
            return Err(anyhow!("No such dotfile: {}", dotpath));
        }
        self.save()
            .with_context(|| format!("Failed to save config after removing {}", dotpath))
    }

    pub fn get_staged(&self) -> HashMap<DotPath, DotfileEntry> {
        self.staged
            .iter()
            .map(|(k, v)| (DotPath::from_path(self, k), v.clone()))
            .collect::<HashMap<DotPath, DotfileEntry>>()
    }
}
