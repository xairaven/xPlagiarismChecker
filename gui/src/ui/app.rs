use crate::config::Config;
use crate::context::Context;
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
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self::set_style(&config, cc);

        let context = Context::new(config);
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
