use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::file_manager::FileManager;

pub struct Toolbar {
    file_manager: Arc<Mutex<FileManager>>,
    address_bar_text: String,
}

impl Toolbar {
    pub fn new(file_manager: Arc<Mutex<FileManager>>) -> Self {
        Self {
            file_manager,
            address_bar_text: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, runtime: &tokio::runtime::Runtime) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Navigation buttons
            let can_go_back = runtime.block_on(async {
                self.file_manager.lock().await.can_go_back().await
            });

            let can_go_forward = runtime.block_on(async {
                self.file_manager.lock().await.can_go_forward().await
            });

            // Back button
            ui.add_enabled_ui(can_go_back, |ui| {
                if ui.button("⬅").on_hover_text("Go Back (Alt+Left)").clicked() {
                    let file_manager = self.file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.go_back().await {
                            log::error!("Failed to go back: {}", e);
                        }
                    });
                }
            });

            // Forward button
            ui.add_enabled_ui(can_go_forward, |ui| {
                if ui.button("➡").on_hover_text("Go Forward (Alt+Right)").clicked() {
                    let file_manager = self.file_manager.clone();
                    runtime.spawn(async move {
                        let mut fm = file_manager.lock().await;
                        if let Err(e) = fm.go_forward().await {
                            log::error!("Failed to go forward: {}", e);
                        }
                    });
                }
            });

            // Up button
            if ui.button("⬆").on_hover_text("Go Up (Alt+Up)").clicked() {
                let file_manager = self.file_manager.clone();
                runtime.spawn(async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.go_up().await {
                        log::error!("Failed to go up: {}", e);
                    }
                });
            }

            ui.separator();

            // Home button
            if ui.button("🏠").on_hover_text("Go Home").clicked() {
                let file_manager = self.file_manager.clone();
                runtime.spawn(async move {
                    let mut fm = file_manager.lock().await;
                    if let Some(home) = dirs::home_dir() {
                        if let Err(e) = fm.navigate_to(&home).await {
                            log::error!("Failed to navigate to home: {}", e);
                        }
                    }
                });
            }

            ui.separator();

            // Address bar
            let current_path = runtime.block_on(async {
                self.file_manager.lock().await.get_current_path().await
            });

            if self.address_bar_text.is_empty() || self.address_bar_text != current_path.display().to_string() {
                self.address_bar_text = current_path.display().to_string();
            }

            ui.label("📁");
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.address_bar_text)
                    .desired_width(ui.available_width() - 200.0)
                    .hint_text("Enter path...")
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let path = std::path::PathBuf::from(&self.address_bar_text);
                let file_manager = self.file_manager.clone();
                runtime.spawn(async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.navigate_to(&path).await {
                        log::error!("Failed to navigate to path: {}", e);
                    }
                });
            }

            ui.separator();

            // View options
            if ui.button("🔄").on_hover_text("Refresh (F5)").clicked() {
                let file_manager = self.file_manager.clone();
                runtime.spawn(async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.refresh_items().await {
                        log::error!("Failed to refresh: {}", e);
                    }
                });
            }

            // Toggle hidden files
            let show_hidden = runtime.block_on(async {
                self.file_manager.lock().await.get_show_hidden().await
            });

            let hidden_button_text = if show_hidden { "👁" } else { "👁‍🗨" };
            if ui.button(hidden_button_text).on_hover_text("Toggle Hidden Files (Ctrl+H)").clicked() {
                let file_manager = self.file_manager.clone();
                runtime.spawn(async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.toggle_show_hidden().await {
                        log::error!("Failed to toggle hidden files: {}", e);
                    }
                });
            }

            // Search button
            if ui.button("🔍").on_hover_text("Search").clicked() {
                // TODO: Implement search dialog
            }
        });
    }
}
