use crate::errors::{IoError, ProjectError};
use crate::files::FileType;
use engine::models::ignore::IgnoreList;

pub const ACCEPTED_EXTENSIONS_FILE_TYPE: FileType = FileType::AcceptedExtensions;
pub const IGNORED_DIRECTORIES_FILE_TYPE: FileType = FileType::IgnoredDirectories;

#[derive(Debug, Clone)]
pub struct IgnoreSettings {
    pub accepted_extensions: Vec<String>,
    pub ignored_directories: Vec<String>,
}

impl IgnoreSettings {
    pub fn from_file() -> Result<Self, ProjectError> {
        let extensions = Self::read_file(&ACCEPTED_EXTENSIONS_FILE_TYPE);
        let directories = Self::read_file(&IGNORED_DIRECTORIES_FILE_TYPE);

        if extensions.is_empty() && directories.is_empty() {
            let default_list = IgnoreList::default();
            let settings = Self {
                accepted_extensions: default_list.accepted_extensions,
                ignored_directories: default_list.ignored_directories,
            };
            settings.save_to_file()?;
            return Ok(settings);
        }

        Ok(Self {
            accepted_extensions: extensions,
            ignored_directories: directories,
        })
    }

    fn read_file(file_type: &FileType) -> Vec<String> {
        match file_type.path() {
            Ok(path) => {
                let text = std::fs::read_to_string(&path);
                match text {
                    Ok(text) => text
                        .lines()
                        .map(|line| line.trim().to_string())
                        .filter(|line| !line.is_empty())
                        .collect(),
                    Err(_) => Vec::new(),
                }
            },
            Err(_) => Vec::new(),
        }
    }

    pub fn save_to_file(&self) -> Result<(), ProjectError> {
        let extensions_path = ACCEPTED_EXTENSIONS_FILE_TYPE.path()?;
        let directories_path = IGNORED_DIRECTORIES_FILE_TYPE.path()?;
        for path in [&extensions_path, &directories_path] {
            if let Some(parent_path) = path.parent() {
                std::fs::create_dir_all(parent_path)
                    .map_err(IoError::CreateParentDirectories)?;
            }
        }

        let extensions_data = self.accepted_extensions.join("\n");
        std::fs::write(extensions_path, extensions_data).map_err(IoError::Write)?;

        let directories_data = self.ignored_directories.join("\n");
        std::fs::write(directories_path, directories_data).map_err(IoError::Write)?;

        Ok(())
    }
}
