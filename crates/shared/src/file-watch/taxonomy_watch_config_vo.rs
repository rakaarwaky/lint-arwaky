// PURPOSE: WatchConfig — value object for file watch configuration parameters
use crate::common::taxonomy_path_vo::FilePath;

pub struct WatchConfig {
    pub path: FilePath,
    pub recursive: bool,
    pub debounce_ms: u64,
    pub ignore_patterns: Vec<String>,
}

impl WatchConfig {
    pub fn from_path(path: String) -> Self {
        Self {
            path: FilePath::new(path).unwrap_or_default(),
            recursive: true,
            debounce_ms: 500,
            ignore_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "__pycache__".to_string(),
                "target".to_string(),
                ".venv".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
        }
    }
}
