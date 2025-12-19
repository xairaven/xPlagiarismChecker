use crate::context::Context;
use crate::localization::{Localized, LocalizedLabel};
use crate::ui::commands::UiCommand;
use crate::ui::pages::PageId;
use crate::ui::{Ui, styles};
use egui::{Button, SidePanel};
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
    pub fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let style = &ctx.gui.style;

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
                        ui.add_space(styles::space::NAVIGATOR_HEADER);
                        ui.heading(styles::heading::huge(
                            &LocalizedLabel::NavigationMenu.localize(),
                        ));
                        egui::warn_if_debug_build(ui);
                    },
                );

                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui| {
                        for (tab, label) in &self.tabs {
                            let button =
                                Button::selectable(ctx.active_page() == *tab, label);

                            if ui.add(button).clicked() {
                                self.change_tab(ctx, *tab);
                            }
                        }
                    },
                );
            });
    }

    fn change_tab(&self, ctx: &Context, page_id: PageId) {
        let command = UiCommand::ChangePage(page_id);
        ctx.gui.ui_channel.try_send(command, ctx);
    }
}
