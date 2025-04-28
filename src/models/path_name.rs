use std::{fmt, path::PathBuf};

use crate::utils;

use super::Config;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DotPath {
    pub rel_path: PathBuf,
    pub abs_path: PathBuf,
    pub target: PathBuf,
    pub abs_target: PathBuf,
    pub target_staged: PathBuf,
    pub abs_target_staged: PathBuf,
}

impl DotPath {
    pub fn new(config: &Config, name: &str) -> Self {
        let name = utils::expand_tilde(&name).to_string_lossy().to_string();
        
        DotPath {
            rel_path: utils::reduce_path_to_home(&name),
            abs_path: utils::get_full_path(&name),
            target: utils::replace_home(&name, config.repo_path.to_str().unwrap()),
            abs_target: utils::get_full_path(
                utils::replace_home(&name, config.repo_path.to_str().unwrap())
                    .to_string_lossy()
                    .as_ref(),
            ),
            target_staged: utils::replace_home(&name, config.staging_path.to_str().unwrap()),
            abs_target_staged: utils::get_full_path(
                utils::replace_home(&name, config.staging_path.to_str().unwrap())
                    .to_string_lossy()
                    .as_ref(),
            ),
        }
    }

    pub fn from_path(config: &Config, path: &PathBuf) -> Self {
        DotPath::new(config, path.to_str().unwrap())
    }

    pub fn to_name(&self) -> &PathBuf {
        &self.rel_path
    }
}

impl fmt::Display for DotPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rel_path.display())
    }
}
