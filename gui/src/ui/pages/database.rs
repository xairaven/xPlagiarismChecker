use crate::context::Context;
use crate::ui::pages::Page;
use egui::Ui;

#[derive(Debug, Default)]
pub struct DatabasePage;

impl Page for DatabasePage {
    fn show_content(&mut self, ui: &mut Ui, ctx: &Context) {
        self.page_header(ui);

        ui.label("TODO: Database page content goes here.");
    }
}
