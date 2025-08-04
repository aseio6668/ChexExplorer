use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    pub icon: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Bookmark {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            icon: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.add_default_bookmarks();
        manager
    }

    fn add_default_bookmarks(&mut self) {
        // Add common system bookmarks
        if let Some(home) = dirs::home_dir() {
            self.bookmarks.push(Bookmark::new("Home".to_string(), home));
        }

        if let Some(desktop) = dirs::desktop_dir() {
            self.bookmarks.push(Bookmark::new("Desktop".to_string(), desktop));
        }

        if let Some(documents) = dirs::document_dir() {
            self.bookmarks.push(Bookmark::new("Documents".to_string(), documents));
        }

        if let Some(downloads) = dirs::download_dir() {
            self.bookmarks.push(Bookmark::new("Downloads".to_string(), downloads));
        }

        if let Some(pictures) = dirs::picture_dir() {
            self.bookmarks.push(Bookmark::new("Pictures".to_string(), pictures));
        }

        if let Some(music) = dirs::audio_dir() {
            self.bookmarks.push(Bookmark::new("Music".to_string(), music));
        }

        if let Some(videos) = dirs::video_dir() {
            self.bookmarks.push(Bookmark::new("Videos".to_string(), videos));
        }

        // Add root drives on Windows
        #[cfg(windows)]
        {
            for drive in 'A'..='Z' {
                let path = PathBuf::from(format!("{}:\\", drive));
                if path.exists() {
                    self.bookmarks.push(Bookmark::new(
                        format!("Drive {}", drive),
                        path,
                    ));
                }
            }
        }

        // Add root on Unix-like systems
        #[cfg(not(windows))]
        {
            self.bookmarks.push(Bookmark::new("Root".to_string(), PathBuf::from("/")));
        }
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        // Check if bookmark already exists
        if !self.bookmarks.iter().any(|b| b.path == bookmark.path) {
            self.bookmarks.push(bookmark);
        }
    }

    pub fn remove_bookmark(&mut self, index: usize) {
        if index < self.bookmarks.len() {
            self.bookmarks.remove(index);
        }
    }

    pub fn get_bookmarks(&self) -> &Vec<Bookmark> {
        &self.bookmarks
    }

    pub fn get_bookmark(&self, index: usize) -> Option<&Bookmark> {
        self.bookmarks.get(index)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let manager: BookmarkManager = serde_json::from_str(&content)?;
        Ok(manager)
    }
}
