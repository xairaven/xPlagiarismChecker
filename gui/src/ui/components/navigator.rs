use crate::localization::{Label, Localized};
use crate::ui::pages::PageId;
use crate::ui::styles;
use egui::SidePanel;
use std::collections::BTreeMap;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Navigator {
    tabs: BTreeMap<PageId, String>,
}

impl Default for Navigator {
    fn default() -> Self {
        let mut tabs: BTreeMap<PageId, String> = BTreeMap::new();

        for page in PageId::iter() {
            let label = page.to_string();
            tabs.insert(page, label);
        }

        Self { tabs }
    }
}

impl Navigator {
    pub fn show_content(&mut self, ui: &mut egui::Ui, active_page: &mut PageId) {
        SidePanel::left("navigator_panel")
            .resizable(false)
            .default_width(170.0)
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.add_space(styles::spacing::XLARGE);

                        ui.heading(styles::heading::huge(
                            &Label::NavigationMenu.localize(),
                        ));

                        ui.add_space(styles::spacing::XLARGE);

                        egui::warn_if_debug_build(ui);

                        ui.add_space(styles::spacing::XLARGE);
                    },
                );

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui| {
                        for (tab, label) in &self.tabs {
                            let is_active = tab == active_page;
                            let button = egui::Button::new(label)
                                .fill(if is_active {
                                    ui.visuals().selection.bg_fill
                                } else {
                                    ui.visuals().widgets.inactive.bg_fill
                                })
                                .min_size([120.0, 40.0].into());

                            if ui.add(button).clicked() {
                                *active_page = *tab;
                            }

                            ui.add_space(styles::spacing::LARGE);
                        }
                    },
                );
            });
    }
}
