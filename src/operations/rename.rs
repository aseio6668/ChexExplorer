use std::path::{Path, PathBuf};
use anyhow::Result;

pub async fn rename_file(old_path: &Path, new_name: &str) -> Result<PathBuf> {
    let parent = old_path.parent()
        .ok_or_else(|| anyhow::anyhow!("Cannot get parent directory"))?;
    
    let new_path = parent.join(new_name);
    
    if new_path.exists() {
        return Err(anyhow::anyhow!("A file with that name already exists"));
    }
    
    std::fs::rename(old_path, &new_path)?;
    Ok(new_path)
}

pub async fn move_file(source: &Path, destination: &Path) -> Result<()> {
    std::fs::rename(source, destination)?;
    Ok(())
}
