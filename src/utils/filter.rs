use crate::models::{DotPath, DotfileEntry};

/// Filtering strategy for dotfiles
#[derive(Debug, Clone, PartialEq)]
pub enum FilterStrategy {
    /// Exact match by name
    Exact,
    /// Substring match by name
    Contains,
}

/// Configuration for filtering dotfiles
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub strategy: FilterStrategy,
    pub case_sensitive: bool,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            strategy: FilterStrategy::Exact,
            case_sensitive: false,
        }
    }
}

/// Filter dotfiles based on the given name and strategy
pub fn filter_dotfiles<'a>(
    dotfiles: impl Iterator<Item = (&'a DotPath, &'a DotfileEntry)>,
    name: Option<&str>,
    config: &FilterConfig,
) -> Vec<(&'a DotPath, &'a DotfileEntry)> {
    if name.is_none() {
        return dotfiles.collect();
    }

    dotfiles
        .filter(|(path, _)| matches_filter(path, name.unwrap(), config))
        .collect()
}

/// Convenience function for exact matching (default behavior)
pub fn filter_dotfiles_exact<'a>(
    dotfiles: impl Iterator<Item = (&'a DotPath, &'a DotfileEntry)>,
    name: Option<&str>,
) -> Vec<(&'a DotPath, &'a DotfileEntry)> {
    filter_dotfiles(dotfiles, name, &FilterConfig::default())
}

/// Convenience function for substring matching
pub fn filter_dotfiles_contains<'a>(
    dotfiles: impl Iterator<Item = (&'a DotPath, &'a DotfileEntry)>,
    name: Option<&str>,
) -> Vec<(&'a DotPath, &'a DotfileEntry)> {
    filter_dotfiles(
        dotfiles,
        name,
        &FilterConfig {
            strategy: FilterStrategy::Contains,
            case_sensitive: false,
        },
    )
}

/// Check if a dotfile matches the given filter criteria
pub fn matches_filter(dotfile_path: &DotPath, filter_name: &str, config: &FilterConfig) -> bool {
    let filter_name = if config.case_sensitive {
        filter_name.to_string()
    } else {
        filter_name.to_lowercase()
    };

    let target_name = if config.case_sensitive {
        dotfile_path.abs_path.to_string_lossy().to_string()
    } else {
        dotfile_path.abs_path.to_string_lossy().to_lowercase()
    };

    match config.strategy {
        FilterStrategy::Exact => {
            target_name.trim_end_matches('/') == filter_name.trim_end_matches('/')
        }
        FilterStrategy::Contains => target_name
            .trim_end_matches('/')
            .contains(&filter_name.trim_end_matches('/')),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Config;
    use std::path::PathBuf;

    fn create_test_dotpath(name: &str) -> DotPath {
        let config = Config::new(PathBuf::from("/tmp/test"));
        DotPath::new(&config, name)
    }

    #[test]
    fn test_exact_match() {
        let config = FilterConfig {
            strategy: FilterStrategy::Exact,
            case_sensitive: false,
        };

        let dotpath = create_test_dotpath("vimrc");

        assert!(matches_filter(&dotpath, "vimrc", &config));
        assert!(matches_filter(&dotpath, "VIMRC", &config));
        assert!(!matches_filter(&dotpath, "vim", &config));
        assert!(!matches_filter(&dotpath, "vimrc_backup", &config));
    }

    #[test]
    fn test_contains_match() {
        let config = FilterConfig {
            strategy: FilterStrategy::Contains,
            case_sensitive: false,
        };

        let dotpath = create_test_dotpath("vimrc");

        assert!(matches_filter(&dotpath, "vim", &config));
        assert!(matches_filter(&dotpath, "VIM", &config));
        assert!(matches_filter(&dotpath, "mrc", &config));
        assert!(matches_filter(&dotpath, "vimrc", &config));
        assert!(!matches_filter(&dotpath, "emacs", &config));
    }

    #[test]
    fn test_case_sensitive() {
        let config = FilterConfig {
            strategy: FilterStrategy::Exact,
            case_sensitive: true,
        };

        let dotpath = create_test_dotpath("vimrc");

        assert!(matches_filter(&dotpath, "vimrc", &config));
        assert!(!matches_filter(&dotpath, "VIMRC", &config));
        assert!(!matches_filter(&dotpath, "Vimrc", &config));
    }
}
