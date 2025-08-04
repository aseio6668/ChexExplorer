use std::path::{Path, PathBuf};
use anyhow::Result;

pub async fn create_folder(parent: &Path, name: &str) -> Result<PathBuf> {
    let new_path = parent.join(name);
    
    if new_path.exists() {
        return Err(anyhow::anyhow!("A folder with that name already exists"));
    }
    
    std::fs::create_dir(&new_path)?;
    Ok(new_path)
}

pub async fn create_file(parent: &Path, name: &str) -> Result<PathBuf> {
    let new_path = parent.join(name);
    
    if new_path.exists() {
        return Err(anyhow::anyhow!("A file with that name already exists"));
    }
    
    std::fs::write(&new_path, "")?;
    Ok(new_path)
}
