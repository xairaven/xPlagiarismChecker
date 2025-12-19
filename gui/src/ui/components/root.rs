use crate::context::Context;
use crate::ui::state::GuiState;
use egui::CentralPanel;

pub struct Root;

impl Root {
    pub fn show_content(ui: &mut egui::Ui, context: &Context, state: &mut GuiState) {
        // Navigation panel
        state.show_navigation(ui, context);

        // Central block (for pages)
        let style = &context.gui.style;
        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .inner_margin(style.theme.margin_style())
                    .fill(style.theme.bg_primary_color_visuals()),
            )
            .show(ui.ctx(), |ui| {
                state.show_active_page(ui, context);
            });
    }
}
