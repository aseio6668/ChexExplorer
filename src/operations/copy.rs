use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CopyProgress {
    pub current_file: PathBuf,
    pub total_files: usize,
    pub completed_files: usize,
    pub bytes_copied: u64,
    pub total_bytes: u64,
}

pub struct CopyOperation {
    source_paths: Vec<PathBuf>,
    destination: PathBuf,
    overwrite: bool,
    progress_tx: Option<mpsc::UnboundedSender<CopyProgress>>,
}

impl CopyOperation {
    pub fn new(source_paths: Vec<PathBuf>, destination: PathBuf) -> Self {
        Self {
            source_paths,
            destination,
            overwrite: false,
            progress_tx: None,
        }
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn with_progress_callback(mut self, tx: mpsc::UnboundedSender<CopyProgress>) -> Self {
        self.progress_tx = Some(tx);
        self
    }

    pub async fn execute(&self) -> Result<()> {
        if !self.destination.exists() {
            fs::create_dir_all(&self.destination)?;
        }

        let mut total_files = 0;
        let mut total_bytes = 0;

        // Calculate total work
        for source in &self.source_paths {
            let (files, bytes) = self.calculate_work(source)?;
            total_files += files;
            total_bytes += bytes;
        }

        let mut completed_files = 0;
        let mut bytes_copied = 0;

        // Copy files
        for source in &self.source_paths {
            self.copy_recursive(
                source,
                &self.destination,
                &mut completed_files,
                &mut bytes_copied,
                total_files,
                total_bytes,
            ).await?;
        }

        Ok(())
    }

    fn calculate_work(&self, path: &Path) -> Result<(usize, u64)> {
        let mut files = 0;
        let mut bytes = 0;

        if path.is_file() {
            files = 1;
            bytes = path.metadata()?.len();
        } else if path.is_dir() {
            for entry in walkdir::WalkDir::new(path) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    files += 1;
                    bytes += entry.metadata()?.len();
                }
            }
        }

        Ok((files, bytes))
    }

    fn copy_recursive<'a>(
        &'a self,
        source: &'a Path,
        dest_dir: &'a Path,
        completed_files: &'a mut usize,
        bytes_copied: &'a mut u64,
        total_files: usize,
        total_bytes: u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let dest_name = source.file_name().unwrap();
            let dest_path = dest_dir.join(dest_name);

            if source.is_file() {
                self.copy_file(source, &dest_path).await?;
                *completed_files += 1;
                *bytes_copied += source.metadata()?.len();
                
                if let Some(ref tx) = self.progress_tx {
                    let progress = CopyProgress {
                        current_file: source.to_path_buf(),
                        total_files,
                        completed_files: *completed_files,
                        bytes_copied: *bytes_copied,
                        total_bytes,
                    };
                    let _ = tx.send(progress);
                }
            } else if source.is_dir() {
                fs::create_dir_all(&dest_path)?;

                for entry in fs::read_dir(source)? {
                    let entry = entry?;
                    let entry_path = entry.path();
                    
                    self.copy_recursive(
                        &entry_path,
                        &dest_path,
                        completed_files,
                        bytes_copied,
                        total_files,
                        total_bytes,
                    ).await?;
                }
            }

            Ok(())
        })
    }

    async fn copy_file(&self, source: &Path, dest: &Path) -> Result<()> {
        if dest.exists() && !self.overwrite {
            return Err(anyhow::anyhow!("Destination file already exists: {}", dest.display()));
        }

        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(source, dest)?;
        Ok(())
    }
}

pub async fn copy_files(
    source_paths: Vec<PathBuf>,
    destination: PathBuf,
    overwrite: bool,
    progress_callback: Option<mpsc::UnboundedSender<CopyProgress>>,
) -> Result<()> {
    let mut operation = CopyOperation::new(source_paths, destination)
        .with_overwrite(overwrite);

    if let Some(callback) = progress_callback {
        operation = operation.with_progress_callback(callback);
    }

    operation.execute().await
}
