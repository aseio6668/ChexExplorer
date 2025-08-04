use std::path::{Path, PathBuf};
use anyhow::Result;

pub async fn delete_files(paths: Vec<PathBuf>, use_trash: bool) -> Result<()> {
    for path in paths {
        if use_trash {
            // Move to trash/recycle bin
            trash::delete(&path)?;
        } else {
            // Permanently delete
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

pub async fn delete_file(path: &Path, use_trash: bool) -> Result<()> {
    delete_files(vec![path.to_path_buf()], use_trash).await
}
