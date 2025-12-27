use crate::context::Context;
use crate::ui::state::GuiState;
use egui::CentralPanel;

pub struct Root;

impl Root {
    pub fn show_content(ui: &mut egui::Ui, context: &Context, state: &mut GuiState) {
        // Navigation panel
        state.show_navigation(ui, context);

        // Central block (for pages)
        let theme = context.settings.theme.get_converted();
        CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .inner_margin(theme.margin_style())
                    .fill(theme.bg_primary_color_visuals()),
            )
            .show(ui.ctx(), |ui| {
                state.show_active_page(ui, context);
            });
    }
}
