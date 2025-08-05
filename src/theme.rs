use eframe::egui::{Color32, FontFamily, FontId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub background_color: [u8; 3],
    pub text_color: [u8; 3],
    pub line_number_color: [u8; 3],
    pub selection_color: [u8; 3],
    pub cursor_color: [u8; 3],
    pub border_color: [u8; 3],
    pub menu_bg_color: [u8; 3],
    pub button_color: [u8; 3],
}

impl Default for Theme {
    fn default() -> Self {
        // Renaissance-inspired color palette with warm, elegant tones
        Self {
            background_color: [245, 240, 230], // Warm parchment
            text_color: [40, 30, 20],          // Dark brown
            line_number_color: [120, 100, 80], // Muted bronze
            selection_color: [180, 150, 120],  // Warm tan
            cursor_color: [80, 50, 30],        // Dark wood
            border_color: [160, 130, 100],     // Antique gold
            menu_bg_color: [235, 225, 210],    // Light parchment
            button_color: [200, 170, 140],     // Aged paper
        }
    }
}

impl Theme {
    pub fn dark_renaissance() -> Self {
        Self {
            background_color: [35, 30, 25],    // Dark wood
            text_color: [220, 210, 190],       // Cream
            line_number_color: [140, 120, 100], // Muted gold
            selection_color: [80, 60, 40],     // Dark bronze
            cursor_color: [200, 180, 150],     // Light gold
            border_color: [100, 80, 60],       // Bronze
            menu_bg_color: [45, 40, 35],       // Darker wood
            button_color: [70, 60, 50],        // Dark bronze
        }
    }

    pub fn royal_blue() -> Self {
        Self {
            background_color: [240, 245, 250], // Light blue-white
            text_color: [20, 30, 60],          // Dark navy
            line_number_color: [80, 100, 140], // Royal blue
            selection_color: [150, 170, 200],  // Light royal blue
            cursor_color: [40, 60, 120],       // Deep blue
            border_color: [120, 140, 180],     // Medium blue
            menu_bg_color: [230, 235, 245],    // Very light blue
            button_color: [180, 200, 230],     // Soft blue
        }
    }

    pub fn forest_green() -> Self {
        Self {
            background_color: [240, 245, 240], // Light green-white
            text_color: [20, 40, 20],          // Dark forest green
            line_number_color: [80, 120, 80],  // Forest green
            selection_color: [150, 180, 150],  // Light green
            cursor_color: [40, 80, 40],        // Deep green
            border_color: [120, 160, 120],     // Medium green
            menu_bg_color: [235, 245, 235],    // Very light green
            button_color: [180, 210, 180],     // Soft green
        }
    }

    pub fn to_color32(&self, color: [u8; 3]) -> Color32 {
        Color32::from_rgb(color[0], color[1], color[2])
    }

    pub fn background(&self) -> Color32 {
        self.to_color32(self.background_color)
    }

    pub fn text(&self) -> Color32 {
        self.to_color32(self.text_color)
    }

    pub fn selection(&self) -> Color32 {
        self.to_color32(self.selection_color)
    }

    pub fn cursor(&self) -> Color32 {
        self.to_color32(self.cursor_color)
    }

    pub fn border(&self) -> Color32 {
        self.to_color32(self.border_color)
    }

    pub fn menu_bg(&self) -> Color32 {
        self.to_color32(self.menu_bg_color)
    }

    pub fn button(&self) -> Color32 {
        self.to_color32(self.button_color)
    }

    // Get a contrasting text color for better readability in settings
    pub fn settings_text(&self) -> Color32 {
        // Use a lighter color that contrasts well with the menu background
        match self.menu_bg_color {
            // For light backgrounds, use a medium-dark color
            [r, g, b] if r > 200 && g > 200 && b > 200 => Color32::from_rgb(60, 60, 60),
            // For dark backgrounds, use a light color
            _ => Color32::from_rgb(220, 220, 220),
        }
    }
}

pub fn get_available_fonts() -> Vec<String> {
    vec![
        "Monospace".to_string(),
        "Proportional".to_string(),
        "Consolas".to_string(),
        "Courier New".to_string(),
        "Liberation Mono".to_string(),
        "DejaVu Sans Mono".to_string(),
        "Ubuntu Mono".to_string(),
        "Source Code Pro".to_string(),
        "Fira Code".to_string(),
        "JetBrains Mono".to_string(),
        "Cascadia Code".to_string(),
        "Inconsolata".to_string(),
    ]
}

pub fn create_font_id(font_name: &str, size: f32) -> FontId {
    match font_name {
        "Monospace" => FontId::new(size, FontFamily::Monospace),
        "Proportional" => FontId::new(size, FontFamily::Proportional),
        _ => FontId::new(size, FontFamily::Monospace), // Default to monospace for all custom fonts
    }
}
