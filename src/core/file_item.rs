use std::path::{Path, PathBuf};
use std::fs::Metadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Directory,
    RegularFile,
    SymbolicLink,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub file_type: FileType,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub created: Option<DateTime<Utc>>,
    pub accessed: Option<DateTime<Utc>>,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub thumbnail_path: Option<PathBuf>,
    pub is_selected: bool,
}

impl FileItem {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let metadata = std::fs::metadata(path)?;
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let file_type = if metadata.is_dir() {
            FileType::Directory
        } else if metadata.is_file() {
            FileType::RegularFile
        } else if metadata.file_type().is_symlink() {
            FileType::SymbolicLink
        } else {
            FileType::Other
        };

        let extension = path.extension()
            .map(|ext| ext.to_string_lossy().to_string().to_lowercase());
        
        let mime_type = extension.as_ref()
            .map(|ext| mime_guess::from_ext(ext).first_or_octet_stream().to_string());

        let is_hidden = Self::is_hidden_file(path);
        let is_readonly = metadata.permissions().readonly();

        Ok(FileItem {
            name,
            path: path.to_path_buf(),
            file_type,
            size: metadata.len(),
            modified: DateTime::from(metadata.modified()?),
            created: metadata.created().ok().map(DateTime::from),
            accessed: metadata.accessed().ok().map(DateTime::from),
            is_hidden,
            is_readonly,
            extension,
            mime_type,
            icon_path: None,
            thumbnail_path: None,
            is_selected: false,
        })
    }

    pub fn is_image(&self) -> bool {
        matches!(self.extension.as_deref(), 
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | 
            Some("bmp") | Some("webp") | Some("tiff") | Some("svg")
        )
    }

    pub fn is_video(&self) -> bool {
        matches!(self.extension.as_deref(),
            Some("mp4") | Some("avi") | Some("mkv") | Some("mov") | 
            Some("wmv") | Some("flv") | Some("webm") | Some("m4v")
        )
    }

    pub fn is_audio(&self) -> bool {
        matches!(self.extension.as_deref(),
            Some("mp3") | Some("wav") | Some("flac") | Some("aac") | 
            Some("ogg") | Some("wma") | Some("m4a")
        )
    }

    pub fn is_document(&self) -> bool {
        matches!(self.extension.as_deref(),
            Some("pdf") | Some("doc") | Some("docx") | Some("xls") | 
            Some("xlsx") | Some("ppt") | Some("pptx") | Some("txt") |
            Some("rtf") | Some("odt") | Some("ods") | Some("odp")
        )
    }

    pub fn is_archive(&self) -> bool {
        matches!(self.extension.as_deref(),
            Some("zip") | Some("rar") | Some("7z") | Some("tar") | 
            Some("gz") | Some("bz2") | Some("xz") | Some("lzma")
        )
    }

    pub fn get_size_formatted(&self) -> String {
        let metadata = std::fs::metadata(&self.path);
        match metadata {
            Ok(meta) => {
                let size = meta.len();
                crate::utils::format::format_file_size(size)
            }
            Err(_) => "Unknown".to_string()
        }
    }

    #[cfg(windows)]
    fn is_hidden_file(path: &Path) -> bool {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
        
        std::fs::metadata(path)
            .map(|metadata| metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0)
            .unwrap_or(false)
    }

    #[cfg(not(windows))]
    fn is_hidden_file(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SortBy {
    Name,
    Size,
    Modified,
    Type,
    Created,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl FileItem {
    pub fn sort_items(items: &mut [FileItem], sort_by: SortBy, sort_order: SortOrder) {
        items.sort_by(|a, b| {
            let comparison = match sort_by {
                SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortBy::Size => a.size.cmp(&b.size),
                SortBy::Modified => a.modified.cmp(&b.modified),
                SortBy::Type => a.file_type.cmp(&b.file_type),
                SortBy::Created => a.created.cmp(&b.created),
            };

            match sort_order {
                SortOrder::Ascending => comparison,
                SortOrder::Descending => comparison.reverse(),
            }
        });

        // Always keep directories first
        items.sort_by_key(|item| match item.file_type {
            FileType::Directory => 0,
            _ => 1,
        });
    }
}

impl PartialOrd for FileType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (FileType::Directory, FileType::Directory) => std::cmp::Ordering::Equal,
            (FileType::Directory, _) => std::cmp::Ordering::Less,
            (_, FileType::Directory) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    }
}
