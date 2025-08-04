mod ui;
mod core;
mod operations;
mod utils;

use eframe::egui;

use ui::app::ChexExplorerApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Initialize logging
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("ChexExplorer - Multi-OS File Explorer"),
        ..Default::default()
    };

    eframe::run_native(
        "ChexExplorer - Multi-OS File Explorer",
        options,
        Box::new(|cc| Ok(Box::new(ChexExplorerApp::new(cc)))),
    )
}
