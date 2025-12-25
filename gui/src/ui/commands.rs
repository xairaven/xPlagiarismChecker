use crate::context::Context;
use crate::localization::Language;
use crate::ui::pages::PageId;

#[derive(Debug, Clone)]
pub enum UiCommand {
    ChangePage(PageId),
    ChangeContextLanguage(Language),
    SaveConfig,
}

#[derive(Debug, Default)]
pub struct UiCommandHandler;

impl UiCommandHandler {
    pub fn handle(&mut self, ui: &mut egui::Ui, ctx: &mut Context) {
        // Getting commands from the channels (in context).
        if let Some(command) = ctx.gui.ui_channel.try_recv() {
            self.process(command, ui, ctx);
        }
    }

    fn process(&mut self, command: UiCommand, ui: &mut egui::Ui, context: &mut Context) {
        match command {
            UiCommand::ChangePage(page_id) => Self::change_page(ui, context, page_id),
            UiCommand::ChangeContextLanguage(_language) => todo!(),
            UiCommand::SaveConfig => todo!(),
        }
    }

    fn change_page(ui: &mut egui::Ui, context: &mut Context, page_id: PageId) {
        context.gui.active_page = page_id;

        if page_id.eq(&PageId::Exit) {
            Self::exit(ui);
        }
    }

    fn exit(ui: &mut egui::Ui) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}
