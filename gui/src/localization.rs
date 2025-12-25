use rust_i18n::t;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub trait Localized {
    fn localize(&self) -> String;
}

#[derive(Debug, Default, Clone, EnumIter, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
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

impl Localized for Language {
    fn localize(&self) -> String {
        match self {
            Self::English => t!("Entity.Language.English").to_string(),
            Self::Ukrainian => t!("Entity.Language.Ukrainian").to_string(),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.localize())
    }
}

pub enum LocalizedLabel {
    NavigationMenu,

    AboutDescription,
    AboutDeveloper,
    AboutCheckGithub,
    AboutLatestRelease,

    ButtonApply,
    ButtonSave,

    SettingsAppHeader,
    SettingsAppLanguage,
    SettingsNoteRestartNeeded,
}

impl Localized for LocalizedLabel {
    fn localize(&self) -> String {
        let tag = match self {
            Self::AboutDescription => "Page.About.Description",
            Self::AboutDeveloper => "Page.About.Developer",
            Self::AboutCheckGithub => "Page.About.CheckGithub",
            Self::AboutLatestRelease => "Page.About.LatestRelease",
            Self::ButtonApply => "Button.Apply",
            Self::ButtonSave => "Button.Save",
            Self::NavigationMenu => "Navigation.Label.Menu",
            Self::SettingsAppHeader => "Page.Settings.App.Header",
            Self::SettingsAppLanguage => "Page.Settings.App.Label.Language",
            Self::SettingsNoteRestartNeeded => "Page.Settings.Note.RestartNeeded",
        };

        t!(tag).to_string()
    }
}
