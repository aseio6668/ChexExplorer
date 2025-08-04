use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::file_manager::FileManager;

pub struct StatusBar {
    file_manager: Arc<Mutex<FileManager>>,
}

impl StatusBar {
    pub fn new(file_manager: Arc<Mutex<FileManager>>) -> Self {
        Self {
            file_manager,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        ui.horizontal(|ui| {
            // Current path info
            let current_path = runtime.block_on(async {
                self.file_manager.lock().await.get_current_path().await
            });

            let items = runtime.block_on(async {
                self.file_manager.lock().await.get_items().await
            });

            let selected_items = runtime.block_on(async {
                self.file_manager.lock().await.get_selected_items().await
            });

            // Item count
            let total_items = items.len();
            let selected_count = selected_items.len();
            
            if selected_count > 0 {
                ui.label(format!("{} of {} items selected", selected_count, total_items));
            } else {
                ui.label(format!("{} items", total_items));
            }

            ui.separator();

            // Selected items size
            if selected_count > 0 {
                let total_size: u64 = selected_items.iter()
                    .filter_map(|&index| items.get(index))
                    .map(|item| item.size)
                    .sum();
                
                let formatted_size = crate::utils::format::format_file_size(total_size);
                ui.label(format!("Selected: {}", formatted_size));
                ui.separator();
            }

            // Current directory size (async calculation)
            let dir_info = self.calculate_directory_info(&items);
            ui.label(format!("Total: {}", dir_info));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Show current path
                ui.label(format!("📁 {}", current_path.display()));
            });
        });
    }

    fn calculate_directory_info(&self, items: &[crate::core::file_item::FileItem]) -> String {
        let total_size: u64 = items.iter()
            .filter(|item| item.file_type == crate::core::file_item::FileType::RegularFile)
            .map(|item| item.size)
            .sum();
        
        let file_count = items.iter()
            .filter(|item| item.file_type == crate::core::file_item::FileType::RegularFile)
            .count();
        
        let dir_count = items.iter()
            .filter(|item| item.file_type == crate::core::file_item::FileType::Directory)
            .count();

        if file_count > 0 || dir_count > 0 {
            let size_str = crate::utils::format::format_file_size(total_size);
            format!("{} files, {} folders ({})", file_count, dir_count, size_str)
        } else {
            "Empty folder".to_string()
        }
    }
}
