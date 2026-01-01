use crate::context::Context;
use crate::ui::components::navigator::Navigator;
use crate::ui::pages::about::AboutPage;
use crate::ui::pages::database::DatabasePage;
use crate::ui::pages::settings::SettingsPage;
use crate::ui::pages::{Page, PageId};

pub struct GuiState {
    pub navigator: Navigator,

    pub about: AboutPage,
    pub database: DatabasePage,
    pub settings: SettingsPage,
}

impl<'a> GuiState {
    pub fn new(ctx: &'a Context) -> Self {
        Self {
            navigator: Navigator::default(),

            about: AboutPage::default(),
            database: DatabasePage,
            settings: SettingsPage::new(ctx),
        }
    }

    pub fn show_navigation(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        self.navigator.show_content(ui, ctx);
    }

    pub fn show_active_page(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        match ctx.gui.active_page {
            PageId::Database => self.database.show_content(ui, ctx),
            PageId::Settings => self.settings.show_content(ui, ctx),
            PageId::About => self.about.show_content(ui, ctx),
            PageId::Exit => {},
        }
    }
}
