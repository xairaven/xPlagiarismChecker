use crate::context::Context;
use crate::ui::styles;
use rust_i18n_derive::Localized;
use strum::EnumIter;

#[derive(
    Debug, Default, Clone, Copy, Localized, PartialEq, Eq, PartialOrd, Ord, EnumIter,
)]
pub enum PageId {
    #[default]
    #[tag("Page.Title.Database")]
    Database,
    #[tag("Page.Title.Settings")]
    Settings,
    #[tag("Page.Title.About")]
    About,
    #[tag("Page.Title.Exit")]
    Exit,
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let localized = self.localize();

        let text = match self {
            Self::Database => {
                format!("{:<5} {}", egui_phosphor::regular::DATABASE, localized)
            },
            Self::Settings => {
                format!("{:<5} {}", egui_phosphor::regular::GEAR, localized)
            },
            Self::About => format!("{:<5} {}", egui_phosphor::regular::INFO, localized),
            Self::Exit => {
                format!("{:<5} {}", egui_phosphor::regular::SIGN_OUT, localized)
            },
        };

        write!(f, "{text}")
    }
}

pub trait Page {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context);
    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }
}

pub mod about;
pub mod database;
pub mod settings;
