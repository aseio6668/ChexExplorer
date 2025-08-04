use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::bookmark::BookmarkManager;
use crate::core::file_manager::FileManager;

pub struct Sidebar {
    bookmark_manager: Arc<Mutex<BookmarkManager>>,
}

impl Sidebar {
    pub fn new(bookmark_manager: Arc<Mutex<BookmarkManager>>) -> Self {
        Self {
            bookmark_manager,
        }
    }

    pub fn show(
        &mut self, 
        ui: &mut egui::Ui, 
        runtime: &tokio::runtime::Runtime,
        file_manager: &Arc<Mutex<FileManager>>
    ) {
        ui.vertical(|ui| {
            ui.heading("Quick Access");
            ui.separator();

            let bookmarks = runtime.block_on(async {
                self.bookmark_manager.lock().await.get_bookmarks().clone()
            });

            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for (index, bookmark) in bookmarks.iter().enumerate() {
                        let button_text = format!("📁 {}", bookmark.name);
                        
                        let button_response = ui.selectable_label(false, &button_text)
                            .on_hover_text(bookmark.path.display().to_string());
                        
                        if button_response.clicked() {
                            let path = bookmark.path.clone();
                            let file_manager = file_manager.clone();
                            runtime.spawn(async move {
                                let mut fm = file_manager.lock().await;
                                if let Err(e) = fm.navigate_to(&path).await {
                                    log::error!("Failed to navigate to bookmark: {}", e);
                                }
                            });
                        }

                        // Context menu for bookmarks
                        button_response.context_menu(|ui| {
                            if ui.button("Remove Bookmark").clicked() {
                                let bookmark_manager = self.bookmark_manager.clone();
                                runtime.spawn(async move {
                                    let mut bm = bookmark_manager.lock().await;
                                    bm.remove_bookmark(index);
                                });
                                ui.close_menu();
                            }
                        });
                    }

                    ui.separator();

                    // Add current location as bookmark
                    if ui.button("+ Add Current Location").clicked() {
                        let bookmark_manager = self.bookmark_manager.clone();
                        let file_manager = file_manager.clone();
                        runtime.spawn(async move {
                            let current_path = {
                                let fm = file_manager.lock().await;
                                fm.get_current_path().await
                            };
                            
                            let folder_name = current_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            
                            let bookmark = crate::core::bookmark::Bookmark::new(
                                folder_name.clone(),
                                current_path
                            );
                            
                            let mut bm = bookmark_manager.lock().await;
                            bm.add_bookmark(bookmark);
                        });
                    }
                });

            ui.separator();

            // Quick system locations
            ui.heading("System");
            
            if let Some(home) = dirs::home_dir() {
                if ui.selectable_label(false, "🏠 Home").clicked() {
                    let file_manager = file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.navigate_to(&home).await {
                            log::error!("Failed to navigate to home: {}", e);
                        }
                    });
                }
            }

            if let Some(desktop) = dirs::desktop_dir() {
                if ui.selectable_label(false, "🖥 Desktop").clicked() {
                    let file_manager = file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.navigate_to(&desktop).await {
                            log::error!("Failed to navigate to desktop: {}", e);
                        }
                    });
                }
            }

            if let Some(documents) = dirs::document_dir() {
                if ui.selectable_label(false, "📄 Documents").clicked() {
                    let file_manager = file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.navigate_to(&documents).await {
                            log::error!("Failed to navigate to documents: {}", e);
                        }
                    });
                }
            }

            if let Some(downloads) = dirs::download_dir() {
                if ui.selectable_label(false, "⬇ Downloads").clicked() {
                    let file_manager = file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.navigate_to(&downloads).await {
                            log::error!("Failed to navigate to downloads: {}", e);
                        }
                    });
                }
            }

            // Show drives on Windows
            #[cfg(windows)]
            {
                ui.separator();
                ui.heading("Drives");
                
                for drive in 'A'..='Z' {
                    let drive_path = std::path::PathBuf::from(format!("{}:\\", drive));
                    if drive_path.exists() {
                        let drive_text = format!("💾 Drive {}", drive);
                        if ui.selectable_label(false, &drive_text).clicked() {
                            let file_manager = file_manager.clone();
                            runtime.spawn(async move {
                                let mut fm = file_manager.lock().await;
                                if let Err(e) = fm.navigate_to(&drive_path).await {
                                    log::error!("Failed to navigate to drive: {}", e);
                                }
                            });
                        }
                    }
                }
            }

            // Show root on Unix-like systems
            #[cfg(not(windows))]
            {
                ui.separator();
                if ui.selectable_label(false, "💻 Root").clicked() {
                    let root_path = std::path::PathBuf::from("/");
                    let file_manager = file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.navigate_to(&root_path).await {
                            log::error!("Failed to navigate to root: {}", e);
                        }
                    });
                }
            }
        });
    }
}
