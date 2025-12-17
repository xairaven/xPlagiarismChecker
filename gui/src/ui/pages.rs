use crate::context::Context;
use crate::localization::Localized;
use rust_i18n::t;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageId {
    #[default]
    Main,
    Settings,
    About,
}

impl PageId {
    pub fn show_content(self, ui: &mut egui::Ui, ctx: &mut Context) {
        match self {
            Self::Main => todo!(),
            Self::Settings => todo!(),
            Self::About => todo!(),
        }
    }
}

impl Localized for PageId {
    fn localize(&self) -> String {
        match self {
            Self::Main => t!("Page.Title.Main").to_string(),
            Self::Settings => t!("Page.Title.Settings").to_string(),
            Self::About => t!("Page.Title.About").to_string(),
        }
    }
}

pub trait Page {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &mut Context);
    fn id(&self) -> PageId;
    fn title(&self) -> String {
        self.id().localize()
    }
}
