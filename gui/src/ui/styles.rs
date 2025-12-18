use crate::ui::themes::Theme;
use egui_aesthetix::Aesthetix;
use std::sync::Arc;

#[derive(Debug)]
pub struct StyleSettings {
    pub theme: Arc<dyn Aesthetix>,
}

impl StyleSettings {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme: theme.into_aesthetix_theme(),
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme.into_aesthetix_theme();
    }
}

pub mod heading {
    pub const HUGE: f32 = 20.0;

    pub fn huge(title: &str) -> egui::RichText {
        egui::RichText::new(title).size(HUGE).strong()
    }
}

pub mod spacing {
    pub const SMALL: f32 = 4.0;
    pub const MEDIUM: f32 = 8.0;
    pub const NORMAL: f32 = 10.0;
    pub const LARGE: f32 = 16.0;
    pub const XLARGE: f32 = 32.0;
}
