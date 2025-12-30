use serde::{Deserialize, Serialize};

const ACCEPTED_EXTENSIONS: &str = include_str!("../../assets/accepted_extensions.txt");
const IGNORED_DIRECTORIES: &str = include_str!("../../assets/ignored_directories.txt");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IgnoreList {
    pub accepted_extensions: Vec<String>,
    pub ignored_directories: Vec<String>,
}

impl Default for IgnoreList {
    fn default() -> Self {
        let accepted_extensions = ACCEPTED_EXTENSIONS
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        let ignored_directories = IGNORED_DIRECTORIES
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Self {
            accepted_extensions,
            ignored_directories,
        }
    }
}
