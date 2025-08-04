use eframe::egui;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: uuid::Uuid,
    pub title: String,
    pub path: PathBuf,
    pub is_active: bool,
}

impl Tab {
    pub fn new(title: String, path: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            path,
            is_active: false,
        }
    }
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active_tab_id: Option<uuid::Uuid>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab_id: None,
        }
    }

    pub fn show_tabs(&mut self, ui: &mut egui::Ui) {
        if self.tabs.is_empty() {
            return;
        }

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let mut tab_to_close = None;
            let mut tab_to_activate = None;

            for tab in &self.tabs {
                let is_active = Some(tab.id) == self.active_tab_id;
                
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 4.0;
                        
                        let tab_text = if tab.title.len() > 20 {
                            format!("{}...", &tab.title[..17])
                        } else {
                            tab.title.clone()
                        };

                        if ui.selectable_label(is_active, &tab_text)
                            .on_hover_text(tab.path.display().to_string())
                            .clicked() 
                        {
                            tab_to_activate = Some(tab.id);
                        }

                        if ui.small_button("✕")
                            .on_hover_text("Close tab")
                            .clicked() 
                        {
                            tab_to_close = Some(tab.id);
                        }
                    });
                });
            }

            // Handle tab activation
            if let Some(tab_id) = tab_to_activate {
                self.activate_tab(tab_id);
            }

            // Handle tab closing
            if let Some(tab_id) = tab_to_close {
                self.close_tab(tab_id);
            }

            ui.separator();

            // New tab button
            if ui.button("+ New Tab").clicked() {
                // TODO: Open new tab with current directory or home
                if let Some(home) = dirs::home_dir() {
                    self.add_tab("Home".to_string(), home);
                }
            }
        });
    }

    pub fn add_tab(&mut self, title: String, path: PathBuf) {
        let tab = Tab::new(title, path);
        let tab_id = tab.id;
        
        self.tabs.push(tab);
        self.activate_tab(tab_id);
    }

    pub fn activate_tab(&mut self, tab_id: uuid::Uuid) {
        self.active_tab_id = Some(tab_id);
        
        // Update active status
        for tab in &mut self.tabs {
            tab.is_active = tab.id == tab_id;
        }
    }

    pub fn close_tab(&mut self, tab_id: uuid::Uuid) {
        if let Some(index) = self.tabs.iter().position(|t| t.id == tab_id) {
            self.tabs.remove(index);
            
            // If this was the active tab, activate another one
            if Some(tab_id) == self.active_tab_id {
                if !self.tabs.is_empty() {
                    let new_active_index = if index > 0 { index - 1 } else { 0 };
                    let new_active_id = self.tabs[new_active_index].id;
                    self.activate_tab(new_active_id);
                } else {
                    self.active_tab_id = None;
                }
            }
        }
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab_id.and_then(|id| {
            self.tabs.iter().find(|t| t.id == id)
        })
    }

    pub fn update_active_tab_path(&mut self, path: PathBuf) {
        if let Some(active_id) = self.active_tab_id {
            if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == active_id) {
                tab.path = path.clone();
                tab.title = path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }
        }
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
