use strum::EnumIter;

pub trait Localized {
    fn localize(&self) -> String;
}

#[derive(Debug, EnumIter, PartialEq)]
pub enum Language {
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
            Self::English => t!("Language.English").to_string(),
            Self::Ukrainian => t!("Language.Ukrainian").to_string(),
        }
    }
}
