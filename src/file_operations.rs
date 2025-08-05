use rfd::{FileDialog, MessageDialog, MessageLevel};
use std::path::{Path, PathBuf};

pub struct FileOperations;

impl FileOperations {
    pub fn open_file() -> Option<(PathBuf, String)> {
        if let Some(path) = FileDialog::new()
            .add_filter("Text Files", &["txt", "md", "rs", "py", "js", "html", "css", "json"])
            .add_filter("All Files", &["*"])
            .pick_file()
        {
            match std::fs::read_to_string(&path) {
                Ok(content) => Some((path, content)),
                Err(e) => {
                    MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error Opening File")
                        .set_description(&format!("Failed to open file: {}", e))
                        .show();
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn save_file(path: &Path, content: &str) -> bool {
        match std::fs::write(path, content) {
            Ok(()) => true,
            Err(e) => {
                MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Error Saving File")
                    .set_description(&format!("Failed to save file: {}", e))
                    .show();
                false
            }
        }
    }

    pub fn save_as_dialog(content: &str, default_name: Option<&str>) -> Option<PathBuf> {
        let mut dialog = FileDialog::new()
            .add_filter("Text Files", &["txt"])
            .add_filter("Markdown Files", &["md"])
            .add_filter("All Files", &["*"]);

        if let Some(name) = default_name {
            dialog = dialog.set_file_name(name);
        }

        if let Some(path) = dialog.save_file() {
            if Self::save_file(&path, content) {
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn save_as_copy_dialog(content: &str, original_path: Option<&Path>) -> Option<PathBuf> {
        let default_name = if let Some(path) = original_path {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("untitled");
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("txt");
            Some(format!("{}_copy.{}", stem, extension))
        } else {
            Some("untitled_copy.txt".to_string())
        };

        let mut dialog = FileDialog::new()
            .add_filter("Text Files", &["txt"])
            .add_filter("Markdown Files", &["md"])
            .add_filter("All Files", &["*"]);

        if let Some(name) = &default_name {
            dialog = dialog.set_file_name(name);
        }

        if let Some(path) = dialog.save_file() {
            if Self::save_file(&path, content) {
                Some(path)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn confirm_unsaved_changes() -> bool {
        let result = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Unsaved Changes")
            .set_description("You have unsaved changes. Do you want to continue without saving?")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();
        
        matches!(result, rfd::MessageDialogResult::Yes)
    }

    pub fn show_about_dialog() {
        MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title("About DNotepadX")
            .set_description(
                "DNotepadX - Renaissance Text Editor v0.1.0\n\n\
                 A beautiful, feature-rich text editor with renaissance-inspired themes.\n\n\
                 Features:\n\
                 • Multiple renaissance-inspired themes\n\
                 • Customizable fonts and colors\n\
                 • Word wrap\n\
                 • Auto-save\n\
                 • Recent files\n\n\
                 Built with Rust and egui."
            )
            .show();
    }
}
