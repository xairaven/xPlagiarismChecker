use crate::context::Context;
use egui::CentralPanel;

pub struct Root;

impl Root {
    pub fn show_content(ui: &mut egui::Ui, context: &mut Context) {
        let style = &context.gui.style;
        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .inner_margin(style.theme.margin_style())
                    .fill(style.theme.bg_primary_color_visuals()),
            )
            .show(ui.ctx(), |ui| {
                context.active_page().show_content(ui, context)
            });
    }
}
