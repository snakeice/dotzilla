use crate::models::{Config, DotPath, DotfileEntry};
use anyhow::Result;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Tracked,
    Staged,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DialogState {
    None,
    AddFile,
    Confirm(String),
}

pub struct App {
    pub config: Config,
    pub repo_path: String,
    pub current_tab: Tab,
    pub tracked_dotfiles: Vec<(DotPath, DotfileEntry)>,
    pub staged_dotfiles: Vec<(DotPath, DotfileEntry)>,
    pub selected_tracked: usize,
    pub selected_staged: usize,
    pub dialog_state: DialogState,
    pub input_text: String,
    pub message: Option<String>,
    pub error_message: Option<String>,
}

impl App {
    pub fn new(repo_path: String) -> Result<Self> {
        let config = Config::load(&std::path::Path::new(&repo_path))?;
        let mut app = Self {
            config,
            repo_path,
            current_tab: Tab::Tracked,
            tracked_dotfiles: Vec::new(),
            staged_dotfiles: Vec::new(),
            selected_tracked: 0,
            selected_staged: 0,
            dialog_state: DialogState::None,
            input_text: String::new(),
            message: None,
            error_message: None,
        };
        app.refresh()?;
        Ok(app)
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.config = Config::load(&std::path::Path::new(&self.repo_path))?;

        self.tracked_dotfiles = self.config.get().into_iter().collect();
        self.tracked_dotfiles
            .sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));

        self.staged_dotfiles = self.config.get_staged().into_iter().collect();
        self.staged_dotfiles
            .sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));

        // Adjust selection if out of bounds
        if self.selected_tracked >= self.tracked_dotfiles.len() && !self.tracked_dotfiles.is_empty()
        {
            self.selected_tracked = self.tracked_dotfiles.len() - 1;
        }
        if self.selected_staged >= self.staged_dotfiles.len() && !self.staged_dotfiles.is_empty() {
            self.selected_staged = self.staged_dotfiles.len() - 1;
        }

        Ok(())
    }

    pub fn next(&mut self) {
        match self.current_tab {
            Tab::Tracked => {
                if !self.tracked_dotfiles.is_empty() {
                    self.selected_tracked =
                        (self.selected_tracked + 1) % self.tracked_dotfiles.len();
                }
            }
            Tab::Staged => {
                if !self.staged_dotfiles.is_empty() {
                    self.selected_staged = (self.selected_staged + 1) % self.staged_dotfiles.len();
                }
            }
            Tab::Help => {}
        }
    }

    pub fn previous(&mut self) {
        match self.current_tab {
            Tab::Tracked => {
                if !self.tracked_dotfiles.is_empty() {
                    self.selected_tracked = if self.selected_tracked == 0 {
                        self.tracked_dotfiles.len() - 1
                    } else {
                        self.selected_tracked - 1
                    };
                }
            }
            Tab::Staged => {
                if !self.staged_dotfiles.is_empty() {
                    self.selected_staged = if self.selected_staged == 0 {
                        self.staged_dotfiles.len() - 1
                    } else {
                        self.selected_staged - 1
                    };
                }
            }
            Tab::Help => {}
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Tracked => Tab::Staged,
            Tab::Staged => Tab::Help,
            Tab::Help => Tab::Tracked,
        };
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Tracked => Tab::Help,
            Tab::Staged => Tab::Tracked,
            Tab::Help => Tab::Staged,
        };
    }

    pub fn show_add_dialog(&mut self) {
        self.dialog_state = DialogState::AddFile;
        self.input_text.clear();
    }

    pub fn handle_enter(&mut self) -> Result<()> {
        match &self.dialog_state {
            DialogState::AddFile => {
                if !self.input_text.is_empty() {
                    match self.add_dotfile(self.input_text.clone()) {
                        Ok(_) => {
                            self.message = Some(format!("Added dotfile: {}", self.input_text));
                            self.error_message = None;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Error adding dotfile: {}", e));
                            self.message = None;
                        }
                    }
                    self.dialog_state = DialogState::None;
                    self.input_text.clear();
                    self.refresh()?;
                }
            }
            DialogState::Confirm(action) => {
                if action == "remove" {
                    if let Err(e) = self.remove_selected() {
                        self.error_message = Some(format!("Error removing dotfile: {}", e));
                    } else {
                        self.message = Some("Dotfile removed successfully".to_string());
                    }
                }
                self.dialog_state = DialogState::None;
                self.refresh()?;
            }
            DialogState::None => {
                match self.current_tab {
                    Tab::Staged => {
                        // Toggle staging for selected item
                        self.toggle_stage_selected()?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn handle_escape(&mut self) {
        if self.dialog_state != DialogState::None {
            self.dialog_state = DialogState::None;
            self.input_text.clear();
        }
        self.message = None;
        self.error_message = None;
    }

    pub fn add_dotfile(&mut self, path: String) -> Result<()> {
        let dot_path = DotPath::new(&self.config, &path);
        let config = Config::load(&std::path::Path::new(&self.repo_path))?;
        crate::commands::add_dotfile(config, dot_path)?;
        Ok(())
    }

    pub fn remove_selected(&mut self) -> Result<()> {
        match self.current_tab {
            Tab::Tracked => {
                if let Some((dot_path, _)) = self.tracked_dotfiles.get(self.selected_tracked) {
                    self.config.remove(dot_path.clone())?;
                    self.message = Some(format!("Removed dotfile: {}", dot_path));
                    self.error_message = None;
                }
            }
            Tab::Staged => {
                if let Some((dot_path, _)) = self.staged_dotfiles.get(self.selected_staged) {
                    self.config.unstage(dot_path)?;
                    self.message = Some(format!("Unstaged dotfile: {}", dot_path));
                    self.error_message = None;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn toggle_stage_selected(&mut self) -> Result<()> {
        match self.current_tab {
            Tab::Tracked => {
                if let Some((dot_path, entry)) = self.tracked_dotfiles.get(self.selected_tracked) {
                    self.config.stage(dot_path, entry.clone())?;
                    self.message = Some(format!("Staged dotfile: {}", dot_path));
                    self.error_message = None;
                }
            }
            Tab::Staged => {
                if let Some((dot_path, _)) = self.staged_dotfiles.get(self.selected_staged) {
                    self.config.unstage(dot_path)?;
                    self.message = Some(format!("Unstaged dotfile: {}", dot_path));
                    self.error_message = None;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn link_staged(&mut self) -> Result<()> {
        crate::commands::link_dotfiles(&self.config, None)?;
        self.message = Some("Linked all staged dotfiles".to_string());
        self.error_message = None;
        Ok(())
    }

    pub fn unlink_all(&mut self) -> Result<()> {
        crate::commands::unlink_dotfiles(&self.config, None)?;
        self.message = Some("Unlinked all dotfiles".to_string());
        self.error_message = None;
        Ok(())
    }

    pub fn commit_staged(&mut self) -> Result<()> {
        crate::commands::commit_dotfiles(&mut self.config)?;
        self.message = Some("Committed all staged dotfiles".to_string());
        self.error_message = None;
        self.refresh()?;
        Ok(())
    }

    pub fn handle_char_input(&mut self, c: char) {
        if let DialogState::AddFile = self.dialog_state {
            self.input_text.push(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        if let DialogState::AddFile = self.dialog_state {
            self.input_text.pop();
        }
    }
}
