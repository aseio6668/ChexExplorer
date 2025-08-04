use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub async fn create_zip_archive(files: Vec<PathBuf>, output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    for file_path in files {
        if file_path.is_file() {
            let file_name = file_path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
                .to_string_lossy();
            
            zip.start_file(file_name, zip::write::SimpleFileOptions::default())?;
            let content = std::fs::read(&file_path)?;
            zip.write_all(&content)?;
        } else if file_path.is_dir() {
            add_directory_to_zip(&mut zip, &file_path, &file_path)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn add_directory_to_zip(
    zip: &mut zip::ZipWriter<File>,
    dir_path: &Path,
    base_path: &Path,
) -> Result<()> {
    for entry in walkdir::WalkDir::new(dir_path) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let relative_path = path.strip_prefix(base_path)?;
            zip.start_file(
                relative_path.to_string_lossy(),
                zip::write::SimpleFileOptions::default(),
            )?;
            let content = std::fs::read(path)?;
            zip.write_all(&content)?;
        }
    }
    Ok(())
}

pub async fn create_tar_archive(files: Vec<PathBuf>, output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let mut archive = tar::Builder::new(file);

    for file_path in files {
        if file_path.is_file() {
            let file_name = file_path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
            archive.append_path_with_name(&file_path, file_name)?;
        } else if file_path.is_dir() {
            archive.append_dir_all(".", &file_path)?;
        }
    }

    archive.finish()?;
    Ok(())
}
