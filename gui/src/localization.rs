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
            Self::English => t!("Settings.Language.English").to_string(),
            Self::Ukrainian => t!("Settings.Language.Ukrainian").to_string(),
        }
    }
}
