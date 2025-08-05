#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod settings;
mod theme;
mod file_operations;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "DNotepadX - Renaissance Text Editor",
        options,
        Box::new(|cc| {
            // Load system fonts if available
            let mut fonts = egui::FontDefinitions::default();
            
            // Try to load Consolas font
            if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\consola.ttf") {
                fonts.font_data.insert(
                    "consolas".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "consolas".to_owned());
            }
            
            // Try to load Courier New font
            if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\courbd.ttf") {
                fonts.font_data.insert(
                    "courier_new".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .insert(1, "courier_new".to_owned());
            }
            
            // Set the fonts
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(app::DNotepadX::new()))
        }),
    )
}
