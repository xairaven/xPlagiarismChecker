use crate::context::Context;
use crate::files::AppFiles;
use crate::files::config::Config;
use crate::ui::commands::UiCommandHandler;
use crate::ui::components::root::Root;
use crate::ui::modals::ModalsHandler;
use crate::ui::state::GuiState;
use egui::CentralPanel;

pub struct App {
    pub context: Context,
    pub state: GuiState,

    modals_handler: ModalsHandler,
    ui_command_handler: UiCommandHandler,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, app_files: AppFiles) -> Self {
        Self::set_fonts(cc);
        Self::set_style(&app_files.config, cc);

        let context = Context::new(app_files);
        let state = GuiState::new(&context);

        Self {
            context,
            state,

            modals_handler: Default::default(),
            ui_command_handler: Default::default(),
        }
    }

    fn set_style(config: &Config, cc: &eframe::CreationContext<'_>) {
        let style = config.theme.into_aesthetix_theme().custom_style();
        cc.egui_ctx.set_style(style);
    }

    fn set_fonts(cc: &eframe::CreationContext<'_>) {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Root::show_content(ui, &self.context, &mut self.state);

            self.ui_command_handler.handle(ui, &mut self.context);
            self.modals_handler.handle_errors(ui, &self.context);
        });
    }
}
