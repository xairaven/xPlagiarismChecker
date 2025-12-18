use crate::config::Config;
use crate::context::Context;
use crate::ui::modals::Modal;
use crate::ui::modals::error::ErrorModal;

pub struct App {
    context: Context,

    errors: Vec<ErrorModal>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        let ctx = Context::new(config);

        let style = ctx.config.theme.into_aesthetix_theme().custom_style();
        cc.egui_ctx.set_style(style);

        Self {
            context: ctx,
            errors: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.context.gui.navigator.show_content(
                ui,
                &self.context.gui.style,
                &mut self.context.gui.active_page,
            );

            self.context
                .active_page()
                .show_content(ui, &mut self.context);

            self.check_for_modals(ui);
        });
    }
}

impl App {
    fn check_for_modals(&mut self, ui: &mut egui::Ui) {
        // Getting modals from the channels (in context).
        if let Ok(modal) = self.context.gui.errors_rx.try_recv() {
            self.errors.push(modal);
        }

        // Showing modals.
        self.show_opened_modals(ui);
    }

    fn show_opened_modals(&mut self, ui: &mut egui::Ui) {
        let mut closed_modals: Vec<usize> = vec![];

        for (index, modal) in self.errors.iter_mut().enumerate() {
            modal.show_content(ui, &mut self.context);

            if modal.is_closed() {
                closed_modals.push(index);
            }
        }

        closed_modals.iter().for_each(|index| {
            self.errors.remove(*index);
        });
    }
}
