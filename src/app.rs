use crate::file_operations::FileOperations;
use crate::settings::Settings;
use crate::theme::{create_font_id, get_available_fonts, Theme};
use eframe::egui::{self, Margin, Rounding, Stroke, TextEdit, Ui};
use std::path::PathBuf;

pub struct DNotepadX {
    settings: Settings,
    content: String,
    current_file: Option<PathBuf>,
    is_modified: bool,
    show_settings: bool,
    show_find_replace: bool,
    find_text: String,
    replace_text: String,
    temp_theme: Theme,
    temp_font_family: String,
    temp_font_size: f32,
    temp_word_wrap: bool,
    cursor_position: Option<usize>,
}

impl DNotepadX {
    pub fn new() -> Self {
        let settings = Settings::load();
        Self {
            temp_theme: settings.theme.clone(),
            temp_font_family: settings.font_family.clone(),
            temp_font_size: settings.font_size,
            temp_word_wrap: settings.word_wrap,
            settings,
            content: String::new(),
            current_file: None,
            is_modified: false,
            show_settings: false,
            show_find_replace: false,
            find_text: String::new(),
            replace_text: String::new(),
            cursor_position: None,
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::default();
        
        // Set overall theme colors
        visuals.window_fill = self.settings.theme.background();
        visuals.panel_fill = self.settings.theme.menu_bg();
        visuals.faint_bg_color = self.settings.theme.button();
        visuals.extreme_bg_color = self.settings.theme.background();
        visuals.code_bg_color = self.settings.theme.background();
        
        // Text colors - use contrasting text for better readability
        let contrasting_text_color = self.settings.theme.settings_text();
        visuals.override_text_color = Some(contrasting_text_color);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, contrasting_text_color);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, contrasting_text_color);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, contrasting_text_color);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, contrasting_text_color);
        
        // Button and widget colors
        visuals.widgets.noninteractive.bg_fill = self.settings.theme.button();
        visuals.widgets.inactive.bg_fill = self.settings.theme.button();
        visuals.widgets.hovered.bg_fill = self.settings.theme.selection();
        visuals.widgets.active.bg_fill = self.settings.theme.selection();
        
        // Border colors
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, self.settings.theme.border());
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.settings.theme.border());
        visuals.widgets.hovered.bg_stroke = Stroke::new(2.0, self.settings.theme.border());
        visuals.widgets.active.bg_stroke = Stroke::new(2.0, self.settings.theme.border());
        
        // Selection colors
        visuals.selection.bg_fill = self.settings.theme.selection();
        visuals.selection.stroke = Stroke::new(1.0, self.settings.theme.cursor());
        
        // Window styling
        visuals.window_rounding = Rounding::same(8.0);
        visuals.menu_rounding = Rounding::same(6.0);
        visuals.button_frame = true;
        
        ctx.set_visuals(visuals);
    }

    fn menu_bar(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("📄 New").clicked() {
                    self.new_file();
                    ui.close_menu();
                }
                if ui.button("📂 Open").clicked() {
                    self.open_file();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("💾 Save").clicked() {
                    self.save_file();
                    ui.close_menu();
                }
                if ui.button("💾 Save As").clicked() {
                    self.save_as_file();
                    ui.close_menu();
                }
                if ui.button("📋 Save As Copy").clicked() {
                    self.save_as_copy();
                    ui.close_menu();
                }
                ui.separator();
                
                if !self.settings.recent_files.is_empty() {
                    ui.menu_button("📚 Recent Files", |ui| {
                        for file in self.settings.recent_files.clone() {
                            if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
                                if ui.button(name).clicked() {
                                    self.open_recent_file(file);
                                    ui.close_menu();
                                }
                            }
                        }
                    });
                    ui.separator();
                }
                
                if ui.button("🚪 Exit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("🔍 Find & Replace").clicked() {
                    self.show_find_replace = !self.show_find_replace;
                    ui.close_menu();
                }
            });

            ui.menu_button("View", |ui| {
                if ui.button("⚙️ Settings").clicked() {
                    self.show_settings = !self.show_settings;
                    ui.close_menu();
                }
                ui.separator();
                
                ui.menu_button("🎨 Themes", |ui| {
                    if ui.button("📜 Classic Renaissance").clicked() {
                        self.settings.theme = Theme::default();
                        self.temp_theme = self.settings.theme.clone();
                        self.settings.save();
                        ui.close_menu();
                    }
                    if ui.button("🌙 Dark Renaissance").clicked() {
                        self.settings.theme = Theme::dark_renaissance();
                        self.temp_theme = self.settings.theme.clone();
                        self.settings.save();
                        ui.close_menu();
                    }
                    if ui.button("👑 Royal Blue").clicked() {
                        self.settings.theme = Theme::royal_blue();
                        self.temp_theme = self.settings.theme.clone();
                        self.settings.save();
                        ui.close_menu();
                    }
                    if ui.button("🌲 Forest Green").clicked() {
                        self.settings.theme = Theme::forest_green();
                        self.temp_theme = self.settings.theme.clone();
                        self.settings.save();
                        ui.close_menu();
                    }
                });
            });

            ui.menu_button("Help", |ui| {
                if ui.button("ℹ️ About").clicked() {
                    FileOperations::show_about_dialog();
                    ui.close_menu();
                }
            });

            // File status
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(ref path) = self.current_file {
                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");
                    let status = if self.is_modified { 
                        format!("● {}", file_name) 
                    } else { 
                        file_name.to_string() 
                    };
                    ui.label(status);
                } else {
                    let status = if self.is_modified { "● Untitled" } else { "Untitled" };
                    ui.label(status);
                }
            });
        });
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        // Apply theme styling to the window
        let mut style = (*ctx.style()).clone();
        
        // Window frame and header styling
        style.visuals.window_fill = self.settings.theme.menu_bg();
        style.visuals.panel_fill = self.settings.theme.menu_bg();
        style.visuals.window_stroke = Stroke::new(2.0, self.settings.theme.border());
        style.visuals.window_shadow.color = egui::Color32::from_black_alpha(50);
        
        // Window title bar and header styling
        style.visuals.widgets.noninteractive.bg_fill = self.settings.theme.menu_bg();
        style.visuals.widgets.inactive.bg_fill = self.settings.theme.menu_bg();
        style.visuals.widgets.hovered.bg_fill = self.settings.theme.button();
        style.visuals.widgets.active.bg_fill = self.settings.theme.button();
        
        // Window title text styling  
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
        style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
        style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
        
        // Override text color globally for this window
        style.visuals.override_text_color = Some(self.settings.theme.settings_text());
        
        ctx.set_style(style);

        egui::Window::new("⚙️ Settings")
            .resizable(true)
            .default_width(400.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                // Apply theme colors to the UI content
                ui.style_mut().visuals.override_text_color = Some(self.settings.theme.settings_text());
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Ensure all text in the settings window uses the contrasting color
                    ui.style_mut().visuals.override_text_color = Some(self.settings.theme.settings_text());
                    
                    // Style all widgets to use the proper colors
                    ui.style_mut().visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                    ui.style_mut().visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                    ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                    ui.style_mut().visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                    
                    ui.colored_label(self.settings.theme.settings_text(), "🎨 Appearance");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.colored_label(self.settings.theme.settings_text(), "Font Family:");
                        egui::ComboBox::from_label("")
                            .selected_text(&self.temp_font_family)
                            .show_ui(ui, |ui| {
                                for font in get_available_fonts() {
                                    ui.selectable_value(&mut self.temp_font_family, font.clone(), font);
                                }
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(self.settings.theme.settings_text(), "Font Size:");
                        ui.add(egui::Slider::new(&mut self.temp_font_size, 8.0..=32.0).suffix("pt"));
                    });

                    ui.add_space(10.0);
                    ui.colored_label(self.settings.theme.settings_text(), "📝 Editor");
                    ui.separator();

                    ui.checkbox(&mut self.temp_word_wrap, "Word wrap");

                    ui.add_space(10.0);
                    ui.colored_label(self.settings.theme.settings_text(), "🎨 Custom Colors");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.colored_label(self.settings.theme.settings_text(), "Background:");
                        let mut bg_color = [
                            self.temp_theme.background_color[0] as f32 / 255.0,
                            self.temp_theme.background_color[1] as f32 / 255.0,
                            self.temp_theme.background_color[2] as f32 / 255.0,
                        ];
                        if ui.color_edit_button_rgb(&mut bg_color).changed() {
                            self.temp_theme.background_color = [
                                (bg_color[0] * 255.0) as u8,
                                (bg_color[1] * 255.0) as u8,
                                (bg_color[2] * 255.0) as u8,
                            ];
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.colored_label(self.settings.theme.settings_text(), "Text Color:");
                        let mut text_color = [
                            self.temp_theme.text_color[0] as f32 / 255.0,
                            self.temp_theme.text_color[1] as f32 / 255.0,
                            self.temp_theme.text_color[2] as f32 / 255.0,
                        ];
                        if ui.color_edit_button_rgb(&mut text_color).changed() {
                            self.temp_theme.text_color = [
                                (text_color[0] * 255.0) as u8,
                                (text_color[1] * 255.0) as u8,
                                (text_color[2] * 255.0) as u8,
                            ];
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Line Numbers:");
                        let mut line_color = [
                            self.temp_theme.line_number_color[0] as f32 / 255.0,
                            self.temp_theme.line_number_color[1] as f32 / 255.0,
                            self.temp_theme.line_number_color[2] as f32 / 255.0,
                        ];
                        if ui.color_edit_button_rgb(&mut line_color).changed() {
                            self.temp_theme.line_number_color = [
                                (line_color[0] * 255.0) as u8,
                                (line_color[1] * 255.0) as u8,
                                (line_color[2] * 255.0) as u8,
                            ];
                        }
                    });

                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        // Style buttons with proper text color
                        let button_style = ui.style_mut();
                        button_style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                        button_style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                        button_style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, self.settings.theme.settings_text());
                        
                        if ui.button("✅ Apply").clicked() {
                            self.settings.theme = self.temp_theme.clone();
                            self.settings.font_family = self.temp_font_family.clone();
                            self.settings.font_size = self.temp_font_size;
                            self.settings.word_wrap = self.temp_word_wrap;
                            self.settings.save();
                        }
                        
                        if ui.button("❌ Cancel").clicked() {
                            self.temp_theme = self.settings.theme.clone();
                            self.temp_font_family = self.settings.font_family.clone();
                            self.temp_font_size = self.settings.font_size;
                            self.temp_word_wrap = self.settings.word_wrap;
                            self.show_settings = false;
                        }

                        if ui.button("🔄 Reset to Default").clicked() {
                            self.temp_theme = Theme::default();
                            self.temp_font_family = "Consolas".to_string();
                            self.temp_font_size = 14.0;
                            self.temp_word_wrap = true;
                        }
                    });
                });
            });
    }

    fn find_replace_window(&mut self, ctx: &egui::Context) {
        if !self.show_find_replace {
            return;
        }

        egui::Window::new("🔍 Find & Replace")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    ui.text_edit_singleline(&mut self.find_text);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Replace:");
                    ui.text_edit_singleline(&mut self.replace_text);
                });

                ui.horizontal(|ui| {
                    if ui.button("Find Next").clicked() {
                        self.find_next();
                    }
                    if ui.button("Replace").clicked() {
                        self.replace_current();
                    }
                    if ui.button("Replace All").clicked() {
                        self.replace_all();
                    }
                });

                if ui.button("Close").clicked() {
                    self.show_find_replace = false;
                }
            });
    }

    fn text_editor(&mut self, ui: &mut Ui) {
        let font_id = create_font_id(&self.settings.font_family, self.settings.font_size);
        
        egui::Frame::none()
            .fill(self.settings.theme.background())
            .stroke(Stroke::new(1.0, self.settings.theme.border()))
            .inner_margin(Margin::same(8.0))
            .show(ui, |ui| {
                ui.style_mut().override_font_id = Some(font_id.clone());
                // For text editor content, use the original theme text color for proper contrast with background
                ui.style_mut().visuals.override_text_color = Some(self.settings.theme.text());
                
                let text_edit = TextEdit::multiline(&mut self.content)
                    .font(font_id.clone())
                    .desired_width(f32::INFINITY)
                    .desired_rows(0)
                    .lock_focus(true);

                let response = ui.add(text_edit);
                
                if response.changed() {
                    self.is_modified = true;
                }
            });
    }

    fn new_file(&mut self) {
        if self.is_modified && !FileOperations::confirm_unsaved_changes() {
            return;
        }
        
        self.content.clear();
        self.current_file = None;
        self.is_modified = false;
    }

    fn open_file(&mut self) {
        if self.is_modified && !FileOperations::confirm_unsaved_changes() {
            return;
        }

        if let Some((path, content)) = FileOperations::open_file() {
            self.content = content;
            self.settings.add_recent_file(path.clone());
            self.current_file = Some(path);
            self.is_modified = false;
            self.settings.save();
        }
    }

    fn open_recent_file(&mut self, path: PathBuf) {
        if self.is_modified && !FileOperations::confirm_unsaved_changes() {
            return;
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                self.content = content;
                self.settings.add_recent_file(path.clone());
                self.current_file = Some(path);
                self.is_modified = false;
                self.settings.save();
            }
            Err(_) => {
                // Remove from recent files if it doesn't exist
                self.settings.recent_files.retain(|p| p != &path);
                self.settings.save();
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.current_file.clone() {
            if FileOperations::save_file(path, &self.content) {
                self.is_modified = false;
            }
        } else {
            self.save_as_file();
        }
    }

    fn save_as_file(&mut self) {
        let default_name = self.current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());
            
        if let Some(path) = FileOperations::save_as_dialog(&self.content, default_name) {
            self.settings.add_recent_file(path.clone());
            self.current_file = Some(path);
            self.is_modified = false;
            self.settings.save();
        }
    }

    fn save_as_copy(&mut self) {
        FileOperations::save_as_copy_dialog(&self.content, self.current_file.as_deref());
    }

    fn find_next(&mut self) {
        if self.find_text.is_empty() {
            return;
        }
        
        let start_pos = self.cursor_position.unwrap_or(0);
        if let Some(pos) = self.content[start_pos..].find(&self.find_text) {
            self.cursor_position = Some(start_pos + pos);
        } else if let Some(pos) = self.content[..start_pos].find(&self.find_text) {
            self.cursor_position = Some(pos);
        }
    }

    fn replace_current(&mut self) {
        if self.find_text.is_empty() {
            return;
        }
        
        if let Some(pos) = self.cursor_position {
            if self.content[pos..].starts_with(&self.find_text) {
                let end_pos = pos + self.find_text.len();
                self.content.replace_range(pos..end_pos, &self.replace_text);
                self.cursor_position = Some(pos + self.replace_text.len());
                self.is_modified = true;
            }
        }
    }

    fn replace_all(&mut self) {
        if self.find_text.is_empty() {
            return;
        }
        
        let new_content = self.content.replace(&self.find_text, &self.replace_text);
        if new_content != self.content {
            self.content = new_content;
            self.is_modified = true;
        }
    }
}

impl eframe::App for DNotepadX {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.text_editor(ui);
        });

        self.settings_window(ctx);
        self.find_replace_window(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.settings.save();
    }
}
