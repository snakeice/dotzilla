use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotfileEntry {
    pub source: PathBuf,
    pub target: PathBuf,
    pub status: DotfileStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DotfileStatus {
    Tracked,
    Staged,
    Untracked,
    Modified,
}

impl DotfileEntry {
    pub fn new(source: PathBuf, target: PathBuf, status: DotfileStatus) -> Self {
        DotfileEntry {
            source,
            target,
            status,
        }
    }

    pub fn set_status(&mut self, status: DotfileStatus) {
        self.status = status;
    }
}
