use crate::context::Context;
use crate::localization::{Language, LocalizedLabel};
use crate::logs::LogLevel;
use crate::ui::commands::UiCommand;
use crate::ui::pages::Page;
use crate::ui::styles;
use crate::ui::themes::Theme;
use crate::ui::widgets::settings::{ComboBoxSetting, SettingWidget};
use egui::Grid;
use rust_i18n_derive::Localized;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SettingsPage {
    language: ComboBoxSetting<Language>,
    log_level: ComboBoxSetting<LogLevel>,
    theme: ComboBoxSetting<Theme>,
}

impl SettingsPage {
    pub fn new(ctx: &Context) -> Self {
        let language =
            ComboBoxSetting::new(&ctx.config.language, Language::iter().collect())
                .with_label(&LocalizedLabel::PageSettingsAppLabelLanguage.localize())
                .takes_effect_after_restart()
                .send_command_on_save(|language: &Language| {
                    UiCommand::ChangeConfigLanguage(*language)
                });

        let log_level =
            ComboBoxSetting::new(&ctx.config.log_level, LogLevel::iter().collect())
                .with_label(&LocalizedLabel::PageSettingsAppLabelLogLevel.localize())
                .takes_effect_after_restart()
                .send_command_on_save(|log_level: &LogLevel| {
                    UiCommand::ChangeConfigLogLevel(*log_level)
                });

        let theme = ComboBoxSetting::new(
            &ctx.settings.theme.get_preference(),
            Theme::iter().collect(),
        )
        .with_label(&LocalizedLabel::PageSettingsAppLabelTheme.localize())
        .send_command_on_save(|theme: &Theme| UiCommand::ChangeTheme(theme.to_owned()));

        Self {
            language,
            log_level,
            theme,
        }
    }
}

impl Page for SettingsPage {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        const COLUMN_WIDTH: f32 = 120.0;

        self.page_header(ui);

        ui.heading(LocalizedLabel::PageSettingsHeader.localize());
        Grid::new("app_settings")
            .num_columns(4)
            .striped(false)
            .max_col_width(COLUMN_WIDTH)
            .min_col_width(COLUMN_WIDTH)
            .show(ui, |ui| {
                self.language.show(ui, &ctx.config.language, ctx);
                self.log_level.show(ui, &ctx.config.log_level, ctx);
                self.theme
                    .show(ui, &ctx.settings.theme.get_preference(), ctx);
            });
    }

    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }
}
