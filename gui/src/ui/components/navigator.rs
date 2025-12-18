use crate::localization::{Label, Localized};
use crate::ui::pages::PageId;
use crate::ui::styles::StyleSettings;
use crate::ui::{Ui, styles};
use egui::SidePanel;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Navigator {
    tabs: BTreeMap<PageId, String>,
    min_width: f32,
}

impl Default for Navigator {
    fn default() -> Self {
        let mut tabs: BTreeMap<PageId, String> = BTreeMap::new();

        for page in PageId::iter() {
            let label = page.to_string();
            tabs.insert(page, label);
        }

        let min_width = Ui::default().min_width * 0.25;

        Self { tabs, min_width }
    }
}

impl Navigator {
    pub fn show_content(
        &mut self, ui: &mut egui::Ui, style: &StyleSettings, active_page: &mut PageId,
    ) {
        SidePanel::left("navigator_panel")
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(style.theme.bg_secondary_color_visuals())
                    .inner_margin(style.theme.margin_style())
                    .stroke(egui::Stroke::new(
                        1.0,
                        style.theme.bg_secondary_color_visuals(),
                    )),
            )
            .min_width(self.min_width)
            .show_separator_line(true)
            .show(ui.ctx(), |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.add_space(styles::spacing::XLARGE);
                        ui.heading(styles::heading::huge(
                            &Label::NavigationMenu.localize(),
                        ));
                        egui::warn_if_debug_build(ui);
                    },
                );

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui| {
                        for (tab, label) in &self.tabs {
                            ui.selectable_value(active_page, *tab, label);
                        }
                    },
                );
            });
    }
}
