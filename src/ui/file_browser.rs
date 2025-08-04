use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::file_manager::FileManager;
use crate::core::file_item::{FileItem, FileType, SortBy, SortOrder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
    Details,
}

pub struct FileBrowser {
    file_manager: Arc<Mutex<FileManager>>,
    view_mode: ViewMode,
    item_size: f32,
    sort_by: SortBy,
    sort_order: SortOrder,
}

impl FileBrowser {
    pub fn new(file_manager: Arc<Mutex<FileManager>>) -> Self {
        Self {
            file_manager,
            view_mode: ViewMode::Details,
            item_size: 64.0,
            sort_by: SortBy::Name,
            sort_order: SortOrder::Ascending,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        ui.vertical(|ui| {
            // View controls
            self.show_view_controls(ui, runtime);
            ui.separator();

            // File list
            match self.view_mode {
                ViewMode::List => self.show_list_view(ui, runtime),
                ViewMode::Grid => self.show_grid_view(ui, runtime),
                ViewMode::Details => self.show_details_view(ui, runtime),
            }
        });
    }

    fn show_view_controls(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        ui.horizontal(|ui| {
            ui.label("View:");
            
            ui.selectable_value(&mut self.view_mode, ViewMode::List, "📋 List");
            ui.selectable_value(&mut self.view_mode, ViewMode::Grid, "⊞ Grid");
            ui.selectable_value(&mut self.view_mode, ViewMode::Details, "📊 Details");

            ui.separator();

            ui.label("Sort by:");
            
            if ui.selectable_value(&mut self.sort_by, SortBy::Name, "Name").clicked() {
                self.update_sort(runtime);
            }
            if ui.selectable_value(&mut self.sort_by, SortBy::Size, "Size").clicked() {
                self.update_sort(runtime);
            }
            if ui.selectable_value(&mut self.sort_by, SortBy::Modified, "Modified").clicked() {
                self.update_sort(runtime);
            }
            if ui.selectable_value(&mut self.sort_by, SortBy::Type, "Type").clicked() {
                self.update_sort(runtime);
            }

            ui.separator();

            let order_text = match self.sort_order {
                SortOrder::Ascending => "🔼",
                SortOrder::Descending => "🔽",
            };

            if ui.button(order_text).clicked() {
                self.sort_order = match self.sort_order {
                    SortOrder::Ascending => SortOrder::Descending,
                    SortOrder::Descending => SortOrder::Ascending,
                };
                self.update_sort(runtime);
            }
        });
    }

    fn update_sort(&self, runtime: &tokio::runtime::Runtime) {
        let file_manager = self.file_manager.clone();
        let sort_by = self.sort_by;
        let sort_order = self.sort_order;
        
        runtime.spawn(async move {
            let mut fm = file_manager.lock().await;
            if let Err(e) = fm.set_sort(sort_by, sort_order).await {
                log::error!("Failed to update sort: {}", e);
            }
        });
    }

    fn show_list_view(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        let items = runtime.block_on(async {
            self.file_manager.lock().await.get_items().await
        });

        let selected_items = runtime.block_on(async {
            self.file_manager.lock().await.get_selected_items().await
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for (index, item) in items.iter().enumerate() {
                    let is_selected = selected_items.contains(&index);
                    let icon = self.get_file_icon(&item);
                    let text = format!("{} {}", icon, item.name);

                    let response = ui.selectable_label(is_selected, &text);
                    
                    self.handle_item_interaction(response, index, item, runtime);
                }
            });
    }

    fn show_grid_view(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        let items = runtime.block_on(async {
            self.file_manager.lock().await.get_items().await
        });

        let selected_items = runtime.block_on(async {
            self.file_manager.lock().await.get_selected_items().await
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                let cols = (ui.available_width() / (self.item_size + 20.0)).floor() as usize;
                if cols == 0 { return; }

                egui::Grid::new("file_grid")
                    .num_columns(cols)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        for (index, item) in items.iter().enumerate() {
                            if index % cols == 0 && index > 0 {
                                ui.end_row();
                            }

                            let is_selected = selected_items.contains(&index);
                            
                            ui.vertical(|ui| {
                                ui.set_width(self.item_size);
                                ui.set_height(self.item_size + 30.0);

                                // File icon/thumbnail
                                let icon = self.get_file_icon(&item);
                                let response = ui.button(
                                    egui::RichText::new(&icon)
                                        .size(self.item_size * 0.6)
                                );

                                // File name
                                ui.label(
                                    egui::RichText::new(&item.name)
                                        .size(10.0)
                                        .color(if is_selected { 
                                            egui::Color32::YELLOW 
                                        } else { 
                                            ui.style().visuals.text_color() 
                                        })
                                );

                                self.handle_item_interaction(response, index, item, runtime);
                            });
                        }
                    });
            });
    }

    fn show_details_view(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        let items = runtime.block_on(async {
            self.file_manager.lock().await.get_items().await
        });

        let selected_items = runtime.block_on(async {
            self.file_manager.lock().await.get_selected_items().await
        });

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                // Header
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.separator();
                    ui.label("Size");
                    ui.separator();
                    ui.label("Type");
                    ui.separator();
                    ui.label("Modified");
                });
                ui.separator();

                // Items
                for (index, item) in items.iter().enumerate() {
                    let is_selected = selected_items.contains(&index);
                    
                    let response = ui.horizontal(|ui| {
                        ui.set_height(20.0);
                        
                        // Icon and name
                        let icon = self.get_file_icon(&item);
                        ui.label(&icon);
                        ui.label(&item.name);
                        
                        ui.separator();
                        
                        // Size
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(item.modified.format("%Y-%m-%d %H:%M").to_string());
                            ui.separator();
                            
                            let file_type = match item.file_type {
                                FileType::Directory => "Folder",
                                FileType::RegularFile => item.extension.as_deref().unwrap_or("File"),
                                FileType::SymbolicLink => "Link",
                                FileType::Other => "Other",
                            };
                            ui.label(file_type);
                            ui.separator();
                            
                            if item.file_type == FileType::Directory {
                                ui.label("-");
                            } else {
                                ui.label(item.get_size_formatted());
                            }
                        });
                    }).response;

                    if is_selected {
                        ui.painter().rect_filled(
                            response.rect,
                            0.0,
                            egui::Color32::from_rgba_unmultiplied(100, 150, 255, 50),
                        );
                    }

                    self.handle_item_interaction(response, index, item, runtime);
                }
            });
    }

    fn handle_item_interaction(
        &self,
        response: egui::Response,
        index: usize,
        item: &FileItem,
        runtime: &tokio::runtime::Runtime,
    ) {
        // Single click - select
        if response.clicked() {
            let file_manager = self.file_manager.clone();
            let ctrl_held = response.ctx.input(|i| i.modifiers.ctrl);
            
            runtime.spawn(async move {
                let fm = file_manager.lock().await;
                fm.select_item(index, ctrl_held).await;
            });
        }

        // Double click - open/navigate
        if response.double_clicked() {
            let path = item.path.clone();
            let file_manager = self.file_manager.clone();
            
            runtime.spawn(async move {
                if path.is_dir() {
                    // Navigate to directory
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.navigate_to(&path).await {
                        log::error!("Failed to navigate to directory: {}", e);
                    }
                } else {
                    // Open file with default application
                    if let Err(e) = open::that(&path) {
                        log::error!("Failed to open file: {}", e);
                    }
                }
            });
        }

        // Context menu
        response.context_menu(|ui| {
            if ui.button("Open").clicked() {
                let path = item.path.clone();
                runtime.spawn(async move {
                    if let Err(e) = open::that(&path) {
                        log::error!("Failed to open file: {}", e);
                    }
                });
                ui.close_menu();
            }

            if item.file_type == FileType::Directory {
                if ui.button("Open in New Tab").clicked() {
                    // TODO: Implement tab functionality
                    ui.close_menu();
                }
            }

            ui.separator();

            if ui.button("Copy").clicked() {
                // TODO: Implement copy to clipboard
                ui.close_menu();
            }

            if ui.button("Cut").clicked() {
                // TODO: Implement cut to clipboard
                ui.close_menu();
            }

            if ui.button("Delete").clicked() {
                let path = item.path.clone();
                runtime.spawn(async move {
                    if let Err(e) = trash::delete(&path) {
                        log::error!("Failed to delete file: {}", e);
                    }
                });
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Rename").clicked() {
                // TODO: Implement rename dialog
                ui.close_menu();
            }

            if ui.button("Properties").clicked() {
                // TODO: Implement properties dialog
                ui.close_menu();
            }
        });
    }

    fn get_file_icon(&self, item: &FileItem) -> String {
        match item.file_type {
            FileType::Directory => "📁".to_string(),
            FileType::RegularFile => {
                if item.is_image() {
                    "🖼".to_string()
                } else if item.is_video() {
                    "🎬".to_string()
                } else if item.is_audio() {
                    "🎵".to_string()
                } else if item.is_document() {
                    "📄".to_string()
                } else if item.is_archive() {
                    "📦".to_string()
                } else {
                    "📄".to_string()
                }
            }
            FileType::SymbolicLink => "🔗".to_string(),
            FileType::Other => "❓".to_string(),
        }
    }
}
