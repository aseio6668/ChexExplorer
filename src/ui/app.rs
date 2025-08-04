use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::file_manager::FileManager;
use crate::core::bookmark::BookmarkManager;
use crate::ui::{toolbar::Toolbar, sidebar::Sidebar, file_browser::FileBrowser, status_bar::StatusBar, tabs::TabManager};

pub struct ChexExplorerApp {
    file_manager: Arc<Mutex<FileManager>>,
    bookmark_manager: Arc<Mutex<BookmarkManager>>,
    toolbar: Toolbar,
    sidebar: Sidebar,
    file_browser: FileBrowser,
    status_bar: StatusBar,
    tab_manager: TabManager,
    runtime: tokio::runtime::Runtime,
}

impl ChexExplorerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup custom fonts if needed
        Self::setup_custom_fonts(&cc.egui_ctx);
        
        // Setup theme
        Self::setup_theme(&cc.egui_ctx);

        let runtime = tokio::runtime::Runtime::new().unwrap();
        let file_manager = Arc::new(Mutex::new(FileManager::new()));
        let bookmark_manager = Arc::new(Mutex::new(BookmarkManager::new()));

        // Initialize with home directory
        {
            let file_manager_clone = file_manager.clone();
            runtime.spawn(async move {
                let mut fm = file_manager_clone.lock().await;
                let home_dir = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"));
                if let Err(e) = fm.navigate_to(&home_dir).await {
                    log::error!("Failed to navigate to home directory: {}", e);
                }
            });
        }

        Self {
            file_manager: file_manager.clone(),
            bookmark_manager: bookmark_manager.clone(),
            toolbar: Toolbar::new(file_manager.clone()),
            sidebar: Sidebar::new(bookmark_manager.clone()),
            file_browser: FileBrowser::new(file_manager.clone()),
            status_bar: StatusBar::new(file_manager.clone()),
            tab_manager: TabManager::new(),
            runtime,
        }
    }

    fn setup_custom_fonts(ctx: &egui::Context) {
        let fonts = egui::FontDefinitions::default();
        
        // Use default fonts for now
        // You can add custom fonts here if needed
        
        ctx.set_fonts(fonts);
    }

    fn setup_theme(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Customize the theme
        style.visuals.window_rounding = egui::Rounding::same(6.0);
        style.visuals.menu_rounding = egui::Rounding::same(4.0);
        
        // Set dark mode by default
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = None;
        
        ctx.set_style(style);
    }
}

impl eframe::App for ChexExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle file system watcher events
        self.handle_file_system_events();

        // Top panel - Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar.show(ui, &self.runtime);
        });

        // Bottom panel - Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.status_bar.show(ui, &self.runtime);
        });

        // Left panel - Sidebar
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(200.0)
            .width_range(150.0..=400.0)
            .show(ctx, |ui| {
                self.sidebar.show(ui, &self.runtime, &self.file_manager);
            });

        // Central panel - Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| {
                self.tab_manager.show_tabs(ui);
            });
            
            ui.separator();

            // File browser
            self.file_browser.show(ui, &self.runtime);
        });

        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        // Request repaint for smooth updates
        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save application state
        if let Ok(bookmark_manager) = self.bookmark_manager.try_lock() {
            if let Ok(serialized) = serde_json::to_string(&*bookmark_manager) {
                storage.set_string("bookmarks", serialized);
            }
        }
    }
}

impl ChexExplorerApp {
    fn handle_file_system_events(&mut self) {
        // Handle file system watcher events in a non-blocking way
        self.runtime.spawn({
            let file_manager = self.file_manager.clone();
            async move {
                let mut fm = file_manager.lock().await;
                let events = fm.check_file_changes();
                
                if !events.is_empty() {
                    // Refresh the file list if there were changes
                    if let Err(e) = fm.refresh_items().await {
                        log::error!("Failed to refresh items after file system event: {}", e);
                    }
                }
            }
        });
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());
        
        // Ctrl+A - Select All
        if input.modifiers.ctrl && input.key_pressed(egui::Key::A) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let fm = file_manager.lock().await;
                    fm.select_all().await;
                }
            });
        }

        // Escape - Clear Selection
        if input.key_pressed(egui::Key::Escape) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let fm = file_manager.lock().await;
                    fm.clear_selection().await;
                }
            });
        }

        // F5 - Refresh
        if input.key_pressed(egui::Key::F5) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.refresh_items().await {
                        log::error!("Failed to refresh: {}", e);
                    }
                }
            });
        }

        // Alt+Left - Go Back
        if input.modifiers.alt && input.key_pressed(egui::Key::ArrowLeft) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.go_back().await {
                        log::error!("Failed to go back: {}", e);
                    }
                }
            });
        }

        // Alt+Right - Go Forward
        if input.modifiers.alt && input.key_pressed(egui::Key::ArrowRight) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.go_forward().await {
                        log::error!("Failed to go forward: {}", e);
                    }
                }
            });
        }

        // Alt+Up - Go Up
        if input.modifiers.alt && input.key_pressed(egui::Key::ArrowUp) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.go_up().await {
                        log::error!("Failed to go up: {}", e);
                    }
                }
            });
        }

        // Ctrl+H - Toggle Hidden Files
        if input.modifiers.ctrl && input.key_pressed(egui::Key::H) {
            self.runtime.spawn({
                let file_manager = self.file_manager.clone();
                async move {
                    let mut fm = file_manager.lock().await;
                    if let Err(e) = fm.toggle_show_hidden().await {
                        log::error!("Failed to toggle hidden files: {}", e);
                    }
                }
            });
        }
    }
}
