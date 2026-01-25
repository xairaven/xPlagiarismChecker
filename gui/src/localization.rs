use egui::WidgetText;
use rust_i18n_derive::Localized;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(
    Debug, Default, Copy, Clone, EnumIter, Localized, PartialEq, Serialize, Deserialize,
)]
pub enum Language {
    #[default]
    #[tag("Entity.Language.English")]
    English,
    #[tag("Entity.Language.Ukrainian")]
    Ukrainian,
}

impl Language {
    pub fn code(&self) -> String {
        let code = match self {
            Self::English => "en",
            Self::Ukrainian => "ua",
        };

        code.to_string()
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.localize())
    }
}

#[derive(Localized)]
pub enum LocalizedLabel {
    #[tag("Page.About.Description")]
    PageAboutDescription,
    #[tag("Page.About.Developer")]
    PageAboutDeveloper,
    #[tag("Page.About.CheckGithub")]
    PageAboutCheckGithub,
    #[tag("Page.About.LatestRelease")]
    PageAboutLatestRelease,
    #[tag("Button.Apply")]
    ButtonApply,
    #[tag("Button.Save")]
    ButtonSave,
    #[tag("Navigation.Label.Menu")]
    NavigationLabelMenu,
    #[tag("Page.Settings.Header")]
    PageSettingsHeader,
    #[tag("Page.Settings.App.Label.Ignore.Directories")]
    PageSettingsAppLabelIgnoredDirectories,
    #[tag("Page.Settings.App.Label.Ignore.Extensions")]
    PageSettingsAppLabelAcceptedExtensions,
    #[tag("Page.Settings.App.Label.Language")]
    PageSettingsAppLabelLanguage,
    #[tag("Page.Settings.App.Label.LogLevel")]
    PageSettingsAppLabelLogLevel,
    #[tag("Page.Settings.App.Label.Theme")]
    PageSettingsAppLabelTheme,
    #[tag("Page.Settings.Note.RestartNeeded")]
    PageSettingsNoteRestartNeeded,
}

impl From<LocalizedLabel> for WidgetText {
    fn from(val: LocalizedLabel) -> Self {
        WidgetText::from(val.localize())
    }
}
