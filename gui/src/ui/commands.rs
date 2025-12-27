use crate::context::Context;
use crate::localization::Language;
use crate::logs::LogLevel;
use crate::ui::modals::error::ErrorModal;
use crate::ui::pages::PageId;
use crate::ui::themes::Theme;

#[derive(Debug, Clone)]
pub enum UiCommand {
    ChangePage(PageId),
    ChangeConfigLogLevel(LogLevel),
    ChangeConfigLanguage(Language),
    ChangeTheme(Theme),
    SaveConfig,
    SynchronizeConfig,
}

#[derive(Debug, Default)]
pub struct UiCommandHandler;

impl UiCommandHandler {
    pub fn handle(&mut self, ui: &mut egui::Ui, ctx: &mut Context) {
        // Getting commands from the channels (in context).
        if let Some(command) = ctx.gui.ui_channel.try_recv() {
            self.process_command(command, ui, ctx);
        }
    }

    fn process_command(
        &mut self, command: UiCommand, ui: &mut egui::Ui, context: &mut Context,
    ) {
        match command {
            UiCommand::ChangeConfigLanguage(language) => {
                Self::change_config_language(context, language)
            },
            UiCommand::ChangeConfigLogLevel(log_level) => {
                Self::change_config_log_level(context, log_level)
            },
            UiCommand::ChangePage(page_id) => Self::change_page(ui, context, page_id),
            UiCommand::ChangeTheme(theme) => Self::change_theme(context, ui, theme),
            UiCommand::SaveConfig => Self::save_config(context),
            UiCommand::SynchronizeConfig => Self::synchronize_config(context),
        }
    }

    fn change_config_language(context: &mut Context, language: Language) {
        context.config.language = language;
    }

    fn change_config_log_level(context: &mut Context, log_level: LogLevel) {
        context.config.log_level = log_level;
    }

    fn change_page(ui: &mut egui::Ui, context: &mut Context, page_id: PageId) {
        context.gui.active_page = page_id;

        if page_id.eq(&PageId::Exit) {
            Self::exit(ui);
        }
    }

    fn change_theme(context: &mut Context, ui: &mut egui::Ui, theme: Theme) {
        context.settings.theme.set(theme);

        let style = theme.into_aesthetix_theme().custom_style();
        ui.ctx().set_style(style);
    }

    fn save_config(context: &mut Context) {
        let save_result = context.config.save_to_file();
        if let Err(e) = save_result {
            log::error!("Failed to save config file: {}", e);
            context.gui.errors_channel.try_send(ErrorModal::new(e));
        }
    }

    fn synchronize_config(context: &mut Context) {
        context.synchronize_config();
    }

    fn exit(ui: &mut egui::Ui) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
}
