use crate::context::Context;
use crate::localization::Localized;
use rust_i18n::t;
use strum::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum PageId {
    #[default]
    Main,
    Settings,
    About,
    Exit,
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let localized = self.localize();

        let text = match self {
            Self::Main => format!("{:<5} {}", "💽", localized),
            Self::Settings => format!("{:<5} {}", "⚙", localized),
            Self::About => format!("{:<5} {}", "ℹ", localized),
            Self::Exit => format!("{:<5} {}", "🗙", localized),
        };

        write!(f, "{text}")
    }
}

impl Localized for PageId {
    fn localize(&self) -> String {
        match self {
            Self::Main => t!("Page.Title.Main").to_string(),
            Self::Settings => t!("Page.Title.Settings").to_string(),
            Self::About => t!("Page.Title.About").to_string(),
            Self::Exit => t!("Page.Title.Exit").to_string(),
        }
    }
}

pub trait Page {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context);
    fn page_header(&self, ui: &mut egui::Ui);
    fn id(&self) -> PageId;
    fn title(&self) -> String {
        self.id().localize()
    }
}

pub mod about;
