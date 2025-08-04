use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs::File;

pub async fn extract_zip(archive_path: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    std::fs::create_dir_all(destination)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = destination.join(file.mangled_name());

        if file.name().ends_with('/') {
            // Directory
            std::fs::create_dir_all(&outpath)?;
        } else {
            // File
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Set permissions on Unix-like systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

pub async fn extract_tar(archive_path: &Path, destination: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = tar::Archive::new(file);

    std::fs::create_dir_all(destination)?;
    archive.unpack(destination)?;

    Ok(())
}

pub async fn extract_archive(archive_path: &Path, destination: &Path) -> Result<()> {
    let extension = archive_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "zip" => extract_zip(archive_path, destination).await,
        "tar" => extract_tar(archive_path, destination).await,
        _ => Err(anyhow::anyhow!("Unsupported archive format: {}", extension)),
    }
}
