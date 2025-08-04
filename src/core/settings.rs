use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub show_hidden_files: bool,
    pub default_view_mode: String,
    pub theme: String,
    pub font_size: f32,
    pub thumbnail_size: u32,
    pub confirm_delete: bool,
    pub confirm_overwrite: bool,
    pub recent_paths: Vec<PathBuf>,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            default_view_mode: "Details".to_string(),
            theme: "Dark".to_string(),
            font_size: 14.0,
            thumbnail_size: 128,
            confirm_delete: true,
            confirm_overwrite: true,
            recent_paths: Vec::new(),
            window_width: 1200.0,
            window_height: 800.0,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("chex-explorer").join("settings.json");
            
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(settings) = serde_json::from_str(&content) {
                    return settings;
                }
            }
        }
        
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("chex-explorer");
            std::fs::create_dir_all(&config_path)?;
            
            let settings_path = config_path.join("settings.json");
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(&settings_path, content)?;
        }
        
        Ok(())
    }

    pub fn add_recent_path(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_paths.retain(|p| p != &path);
        
        // Add to front
        self.recent_paths.insert(0, path);
        
        // Keep only last 10 paths
        self.recent_paths.truncate(10);
    }
}
