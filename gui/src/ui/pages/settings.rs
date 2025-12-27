use crate::context::Context;
use crate::localization::{Language, Localized, LocalizedLabel};
use crate::ui::commands::UiCommand;
use crate::ui::pages::{Page, PageId};
use crate::ui::styles;
use crate::ui::themes::Theme;
use crate::ui::widgets::settings::{ComboBoxSetting, SettingWidget};
use egui::Grid;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct SettingsPage {
    language: ComboBoxSetting<Language>,
    theme: ComboBoxSetting<Theme>,
}

impl SettingsPage {
    pub fn new(ctx: &Context) -> Self {
        let language =
            ComboBoxSetting::new(&ctx.settings.language, Language::iter().collect())
                .with_label(&LocalizedLabel::SettingsAppLanguage.localize())
                .takes_effect_after_restart()
                .send_command_on_save(|language: &Language| {
                    UiCommand::ChangeContextLanguage(*language)
                });

        let theme = ComboBoxSetting::new(
            &ctx.settings.theme.get_preference(),
            Theme::iter().collect(),
        )
        .with_label(&LocalizedLabel::SettingsAppTheme.localize())
        .send_command_on_save(|theme: &Theme| UiCommand::ChangeTheme(theme.to_owned()));

        Self { language, theme }
    }
}

impl Page for SettingsPage {
    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        const COLUMN_WIDTH: f32 = 120.0;

        self.page_header(ui);

        ui.heading(LocalizedLabel::SettingsHeader.localize());
        Grid::new("app_settings")
            .num_columns(4)
            .striped(false)
            .max_col_width(COLUMN_WIDTH)
            .min_col_width(COLUMN_WIDTH)
            .show(ui, |ui| {
                self.language.show(ui, &ctx.settings.language, ctx);
                self.theme
                    .show(ui, &ctx.settings.theme.get_preference(), ctx);
            });
    }

    fn page_header(&self, ui: &mut egui::Ui) {
        ui.add_space(styles::space::PAGE_HEADER);
    }

    fn id(&self) -> PageId {
        PageId::Settings
    }
}
