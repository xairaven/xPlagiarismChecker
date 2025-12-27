use crate::context::Context;
use crate::localization::{Language, Localized, LocalizedLabel};
use crate::ui::commands::UiCommand;
use crate::ui::pages::{Page, PageId};
use crate::ui::styles;
use crate::ui::widgets::settings::{ComboBoxSetting, SettingWidget};
use egui::Grid;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SettingsPage {
    language: ComboBoxSetting<Language>,
}

impl SettingsPage {
    pub fn new(ctx: &Context) -> Self {
        Self {
            language: ComboBoxSetting::new(
                &ctx.settings.language,
                Language::iter().collect(),
            )
            .with_label(&LocalizedLabel::SettingsAppLanguage.localize())
            .takes_effect_after_restart()
            .send_command_on_save(|language: &Language| {
                UiCommand::ChangeContextLanguage(language.clone())
            }),
        }
    }
}

impl Page for SettingsPage {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        const COLUMN_WIDTH: f32 = 120.0;

        self.page_header(ui);

        ui.heading(LocalizedLabel::SettingsAppHeader.localize());
        Grid::new("app_settings")
            .num_columns(4)
            .max_col_width(COLUMN_WIDTH)
            .min_col_width(COLUMN_WIDTH)
            .show(ui, |ui| {
                self.language.show(ui, &ctx.settings.language, ctx);
            });
    }

    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }

    fn id(&self) -> PageId {
        PageId::Settings
    }
}
