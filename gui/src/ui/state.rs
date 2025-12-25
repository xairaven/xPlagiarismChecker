use crate::context::Context;
use crate::ui::components::navigator::Navigator;
use crate::ui::pages::about::AboutPage;
use crate::ui::pages::settings::SettingsPage;
use crate::ui::pages::{Page, PageId};

pub struct GuiState {
    pub about: AboutPage,
    pub settings: SettingsPage,

    pub navigator: Navigator,
}

impl<'a> GuiState {
    pub fn new(ctx: &'a Context) -> Self {
        Self {
            about: AboutPage::default(),
            settings: SettingsPage::new(ctx),

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
            PageId::Settings => self.settings.show_content(ui, ctx),
            PageId::About => self.about.show_content(ui, ctx),
            PageId::Exit => {},
        }
    }
}
