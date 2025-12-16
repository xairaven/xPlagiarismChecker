use serde::{Deserialize, Serialize};
use utils::enum_from_mirror;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

enum_from_mirror!(Theme, egui::ThemePreference, {
    Light,
    Dark,
    System
});
