use crate::context::Context;
use crate::ui::components::navigator::Navigator;
use crate::ui::pages::about::AboutPage;
use crate::ui::pages::{Page, PageId};

pub struct GuiState {
    pub about: AboutPage,

    pub navigator: Navigator,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            about: AboutPage::default(),
            navigator: Navigator::default(),
        }
    }

    pub fn show_navigation(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.navigator.show_content(ui, ctx);
    }

    pub fn show_active_page(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        match ctx.gui.active_page {
            PageId::Main => {
                ui.label("Main");
            },
            PageId::Settings => {
                ui.label("Settings");
            },
            PageId::About => self.about.show_content(ui, ctx),
            PageId::Exit => {},
        }
    }
}
