use std::path::{Path, PathBuf};
use anyhow::Result;
use image::imageops::FilterType;

pub struct ThumbnailGenerator {
    cache_dir: PathBuf,
    thumbnail_size: u32,
}

impl ThumbnailGenerator {
    pub fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("chex-explorer")
            .join("thumbnails");
        
        std::fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            cache_dir,
            thumbnail_size: 128,
        })
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.thumbnail_size = size;
        self
    }

    pub async fn generate_thumbnail(&self, file_path: &Path) -> Result<Option<PathBuf>> {
        if !file_path.exists() || !file_path.is_file() {
            return Ok(None);
        }

        // Generate cache key based on file path and modification time
        let metadata = std::fs::metadata(file_path)?;
        let modified = metadata.modified()?;
        let cache_key = format!(
            "{:x}_{}.png",
            self.hash_path(file_path),
            modified.duration_since(std::time::UNIX_EPOCH)?.as_secs()
        );
        
        let thumbnail_path = self.cache_dir.join(&cache_key);

        // Return cached thumbnail if it exists
        if thumbnail_path.exists() {
            return Ok(Some(thumbnail_path));
        }

        // Generate new thumbnail based on file type
        if let Some(extension) = file_path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            
            match ext.as_str() {
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" => {
                    self.generate_image_thumbnail(file_path, &thumbnail_path).await?;
                }
                "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => {
                    // TODO: Implement video thumbnail generation
                    return Ok(None);
                }
                "pdf" => {
                    // TODO: Implement PDF thumbnail generation
                    return Ok(None);
                }
                _ => {
                    return Ok(None);
                }
            }
        }

        if thumbnail_path.exists() {
            Ok(Some(thumbnail_path))
        } else {
            Ok(None)
        }
    }

    async fn generate_image_thumbnail(&self, source: &Path, destination: &Path) -> Result<()> {
        let img = image::open(source)?;
        let thumbnail = img.resize(self.thumbnail_size, self.thumbnail_size, FilterType::Lanczos3);
        thumbnail.save(destination)?;
        Ok(())
    }

    fn hash_path(&self, path: &Path) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        hasher.finish()
    }

    pub fn clear_cache(&self) -> Result<()> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)?;
            std::fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    pub fn get_cache_size(&self) -> Result<u64> {
        let mut total_size = 0;
        
        if self.cache_dir.exists() {
            for entry in walkdir::WalkDir::new(&self.cache_dir) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    total_size += entry.metadata()?.len();
                }
            }
        }
        
        Ok(total_size)
    }
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            cache_dir: PathBuf::from(".cache/thumbnails"),
            thumbnail_size: 128,
        })
    }
}
