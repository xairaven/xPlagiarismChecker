use egui_aesthetix::Aesthetix;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    StandardDark,
    StandardLight,
    CarlDark,
    NordDark,
    NordLight,
    TokyoNight,
    #[default]
    TokyoNightStorm,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::StandardDark => "Standard Dark",
            Self::StandardLight => "Standard Light",
            Self::CarlDark => "Carl Dark",
            Self::NordDark => "Nord Dark",
            Self::NordLight => "Nord Light",
            Self::TokyoNight => "Tokyo Night",
            Self::TokyoNightStorm => "Tokyo Night Storm",
        };

        write!(f, "{text}")
    }
}

impl Theme {
    pub fn into_aesthetix_theme(self) -> Arc<dyn Aesthetix> {
        match self {
            Self::StandardDark => Arc::new(egui_aesthetix::themes::StandardDark),
            Self::StandardLight => Arc::new(egui_aesthetix::themes::StandardLight),
            Self::CarlDark => Arc::new(egui_aesthetix::themes::CarlDark),
            Self::NordDark => Arc::new(egui_aesthetix::themes::NordDark),
            Self::NordLight => Arc::new(egui_aesthetix::themes::NordLight),
            Self::TokyoNight => Arc::new(egui_aesthetix::themes::TokyoNight),
            Self::TokyoNightStorm => Arc::new(egui_aesthetix::themes::TokyoNightStorm),
        }
    }
}
