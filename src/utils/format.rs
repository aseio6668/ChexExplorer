pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: u64 = 1024;

    if size == 0 {
        return "0 B".to_string();
    }

    let mut size_f = size as f64;
    let mut unit_index = 0;

    while size_f >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size_f /= THRESHOLD as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size_f, UNITS[unit_index])
    }
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

pub fn format_timestamp(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let local: chrono::DateTime<chrono::Local> = timestamp.into();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_path_for_display(path: &std::path::Path, max_length: usize) -> String {
    let path_str = path.display().to_string();
    
    if path_str.len() <= max_length {
        return path_str;
    }

    // Try to keep the filename and some parent directories
    if let Some(filename) = path.file_name() {
        let filename_str = filename.to_string_lossy();
        let remaining_length = max_length.saturating_sub(filename_str.len() + 4); // 4 for ".../"
        
        if remaining_length > 0 {
            let parent = path.parent().unwrap_or(path);
            let parent_str = parent.display().to_string();
            
            if parent_str.len() <= remaining_length {
                return format!("{}/{}", parent_str, filename_str);
            } else {
                let start_pos = parent_str.len().saturating_sub(remaining_length);
                return format!("...{}/{}", &parent_str[start_pos..], filename_str);
            }
        }
    }

    // Fallback: truncate from the beginning
    format!("...{}", &path_str[path_str.len().saturating_sub(max_length - 3)..])
}
