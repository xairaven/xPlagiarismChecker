use std::path::PathBuf;
use strum::{Display, EnumIter, EnumString};
use thiserror::Error;

pub fn open_file(path: PathBuf) -> Result<(), IoError> {
    let _extension = path
        .extension()
        .ok_or(IoError::ExtensionNotFound)?
        .to_os_string()
        .into_string()
        .map_err(|_| IoError::NonUtf8Path)?;

    Ok(())
}

#[derive(Debug, PartialEq, EnumString, Display, EnumIter)]
pub enum SupportedArchiveFormats {
    #[strum(serialize = "zip")]
    Zip,
    #[strum(serialize = "7z")]
    SevenZ,
    #[strum(serialize = "rar")]
    Rar,
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("The file extension is not found.")]
    ExtensionNotFound,

    #[error("The file path contains non-UTF-8 characters.")]
    NonUtf8Path,
}
