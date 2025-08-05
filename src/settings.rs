use crate::theme::Theme;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub font_family: String,
    pub font_size: f32,
    pub word_wrap: bool,
    pub auto_save: bool,
    pub tab_size: usize,
    pub window_width: f32,
    pub window_height: f32,
    pub recent_files: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            font_family: "Monospace".to_string(),
            font_size: 14.0,
            word_wrap: true,
            auto_save: false,
            tab_size: 4,
            window_width: 1200.0,
            window_height: 800.0,
            recent_files: Vec::new(),
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("dnotepadx").join("settings.json");
            if let Ok(content) = std::fs::read_to_string(config_path) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("dnotepadx");
            if std::fs::create_dir_all(&config_path).is_ok() {
                let file_path = config_path.join("settings.json");
                if let Ok(content) = serde_json::to_string_pretty(self) {
                    let _ = std::fs::write(file_path, content);
                }
            }
        }
    }

    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Keep only last 10 files
        self.recent_files.truncate(10);
    }
}
