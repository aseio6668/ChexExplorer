pub fn get_file_icon_unicode(file_name: &str, is_directory: bool) -> &'static str {
    if is_directory {
        return "📁";
    }

    let extension = std::path::Path::new(file_name)
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        // Images
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => "🖼",
        
        // Videos
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "🎬",
        
        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "🎵",
        
        // Documents
        "pdf" => "📄",
        "doc" | "docx" => "📝",
        "xls" | "xlsx" => "📊",
        "ppt" | "pptx" => "📋",
        "txt" | "md" | "rst" => "📄",
        
        // Archives
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "📦",
        
        // Code files
        "rs" => "🦀",
        "py" => "🐍",
        "js" | "ts" => "📜",
        "html" | "htm" => "🌐",
        "css" => "🎨",
        "json" | "xml" | "yaml" | "yml" => "⚙",
        
        // Executables
        "exe" | "msi" | "app" | "deb" | "rpm" => "⚙",
        
        // Default
        _ => "📄",
    }
}

pub fn get_file_type_color(file_name: &str, is_directory: bool) -> (u8, u8, u8) {
    if is_directory {
        return (255, 206, 84); // Yellow for directories
    }

    let extension = std::path::Path::new(file_name)
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        // Images - Purple
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => (186, 85, 211),
        
        // Videos - Red
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => (220, 20, 60),
        
        // Audio - Green
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => (50, 205, 50),
        
        // Documents - Blue
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "rst" => (70, 130, 180),
        
        // Archives - Orange
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => (255, 140, 0),
        
        // Code files - Cyan
        "rs" | "py" | "js" | "ts" | "html" | "htm" | "css" | "json" | "xml" | "yaml" | "yml" => (0, 191, 255),
        
        // Executables - Pink
        "exe" | "msi" | "app" | "deb" | "rpm" => (255, 20, 147),
        
        // Default - Gray
        _ => (169, 169, 169),
    }
}
