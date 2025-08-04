use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn ensure_unique_filename(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }

    let parent = path.parent().unwrap_or(Path::new("."));
    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let extension = path.extension().map(|e| e.to_string_lossy().to_string());

    let mut counter = 1;
    loop {
        let new_name = if let Some(ref ext) = extension {
            format!("{} ({}).{}", file_stem, counter, ext)
        } else {
            format!("{} ({})", file_stem, counter)
        };

        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

pub fn get_file_type_description(path: &Path) -> String {
    if path.is_dir() {
        return "Folder".to_string();
    }

    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_uppercase();
        match ext.as_str() {
            "TXT" => "Text Document".to_string(),
            "PDF" => "PDF Document".to_string(),
            "DOC" | "DOCX" => "Microsoft Word Document".to_string(),
            "XLS" | "XLSX" => "Microsoft Excel Spreadsheet".to_string(),
            "PPT" | "PPTX" => "Microsoft PowerPoint Presentation".to_string(),
            "ZIP" => "ZIP Archive".to_string(),
            "RAR" => "RAR Archive".to_string(),
            "7Z" => "7-Zip Archive".to_string(),
            "JPG" | "JPEG" => "JPEG Image".to_string(),
            "PNG" => "PNG Image".to_string(),
            "GIF" => "GIF Image".to_string(),
            "BMP" => "Bitmap Image".to_string(),
            "SVG" => "SVG Vector Image".to_string(),
            "MP3" => "MP3 Audio".to_string(),
            "WAV" => "Wave Audio".to_string(),
            "MP4" => "MP4 Video".to_string(),
            "AVI" => "AVI Video".to_string(),
            "MKV" => "Matroska Video".to_string(),
            "HTML" | "HTM" => "HTML Document".to_string(),
            "CSS" => "CSS Stylesheet".to_string(),
            "JS" => "JavaScript File".to_string(),
            "JSON" => "JSON File".to_string(),
            "XML" => "XML Document".to_string(),
            "EXE" => "Application".to_string(),
            "DLL" => "Dynamic Link Library".to_string(),
            _ => format!("{} File", ext),
        }
    } else {
        "File".to_string()
    }
}

pub fn is_hidden_file(path: &Path) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
        
        std::fs::metadata(path)
            .map(|metadata| metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0)
            .unwrap_or(false)
    }

    #[cfg(not(windows))]
    {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }
}

pub fn calculate_directory_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            total_size += entry.metadata()?.len();
        }
    }
    
    Ok(total_size)
}

pub fn get_available_drives() -> Vec<PathBuf> {
    let mut drives = Vec::new();
    
    #[cfg(windows)]
    {
        for drive in 'A'..='Z' {
            let path = PathBuf::from(format!("{}:\\", drive));
            if path.exists() {
                drives.push(path);
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        drives.push(PathBuf::from("/"));
        
        // Add common mount points
        let mount_points = [
            "/mnt", "/media", "/Volumes"
        ];
        
        for mount_point in &mount_points {
            let path = PathBuf::from(mount_point);
            if path.exists() {
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        drives.push(entry.path());
                    }
                }
            }
        }
    }
    
    drives
}

pub fn open_file_with_default_app(path: &Path) -> Result<()> {
    open::that(path)?;
    Ok(())
}

pub fn reveal_in_file_manager(path: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path.display().to_string()])
            .spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path.display().to_string()])
            .spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Some(parent) = path.parent() {
            std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn()?;
        }
    }
    
    Ok(())
}
