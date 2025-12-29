use crate::errors::LibError;
use crate::models::database::DatabaseSettings;
use crate::models::submission::{CodeFile, Submission, SubmissionMetadata};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct FileLoader {
    pub submissions: Vec<Submission>,
    pub bad_files: Vec<BadFile>,
}

#[derive(Debug)]
pub struct BadFile {
    pub path: PathBuf,
    pub reason: FileError,
}

#[derive(Debug, Error)]
pub enum FileError {
    #[error("Path does not exist")]
    PathDoesNotExist,

    #[error("File type is unknown or unsupported")]
    UnknownFileType,

    #[error("Failed to open or process archive: {0}")]
    ArchiveError(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("No valid code files found in submission")]
    EmptySubmission,

    #[error("Failed to get file stem from path")]
    FailedToGetStem,

    #[error("Filename does not match the expected pattern")]
    InvalidPattern,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum SupportedArchives {
    Zip,
    Rar,
    SevenZ,
    Tar,
}

impl SupportedArchives {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Zip => "zip",
            Self::Rar => "rar",
            Self::SevenZ => "7z",
            Self::Tar => "tar",
        }
    }

    /// Checks if the file extension corresponds to one of the supported archive types.
    pub fn is_supported(extension: &str) -> bool {
        Self::iter().any(|t| t.extension().eq_ignore_ascii_case(extension))
    }
}

impl FileLoader {
    /// Main entry point. Iterates over provided paths and processes them either as archives or directories.
    pub fn import_submissions(
        paths: Vec<PathBuf>, settings: &DatabaseSettings,
    ) -> Result<Self, LibError> {
        let mut loader = FileLoader::default();

        for path in paths {
            if !path.exists() {
                loader.bad_files.push(BadFile {
                    path,
                    reason: FileError::PathDoesNotExist,
                });
                continue;
            }

            let result = if path.is_dir() {
                Self::process_folder(&path, settings)
            } else if let Some(ext) = path.extension().and_then(|s| s.to_str())
                && SupportedArchives::is_supported(ext)
            {
                Self::process_archive(&path, settings)
            } else {
                Err(FileError::UnknownFileType)
            };

            match result {
                Ok(submission) => loader.submissions.push(submission),
                Err(error) => {
                    loader.bad_files.push(BadFile {
                        path,
                        reason: error,
                    });
                },
            }
        }

        Ok(loader)
    }

    /// Handles archive processing using `compress-tools`.
    /// This supports Zip, Tar, 7z, and Rar uniformly.
    fn process_archive(
        path: &Path, settings: &DatabaseSettings,
    ) -> Result<Submission, FileError> {
        let mut file = File::open(path).map_err(FileError::Io)?;

        // Extract metadata from the archive filename (e.g., "Petrov_Lab1.zip")
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(FileError::FailedToGetStem)?;

        let submission_metadata =
            SubmissionMetadata::parse(filename, &settings.file_name_pattern)
                .map_err(|_| FileError::InvalidPattern)?;
        let mut code_files = Vec::new();

        // Get list of files inside archive
        // We must rewind the file before reading, just in case
        file.seek(SeekFrom::Start(0)).map_err(FileError::Io)?;
        let file_list = compress_tools::list_archive_files(&file)
            .map_err(|error| FileError::ArchiveError(error.to_string()))?;

        // Iterate through files and extract relevant ones
        for entry_name in file_list {
            let path_in_archive = Path::new(&entry_name);

            // Filter 1: Apply Blacklist (Dirs) and Whitelist (Extensions) BEFORE extracting
            // This saves performance by not uncompressing useless files
            if !Self::is_path_allowed(path_in_archive, settings) {
                continue;
            }

            // Important: We must rewind the file source for each extraction call,
            // because `uncompress_data` reads from the stream.
            file.seek(SeekFrom::Start(0)).map_err(FileError::Io)?;

            let mut buffer = Vec::new();

            // Extract specific file content to memory
            match compress_tools::uncompress_archive_file(&file, &mut buffer, &entry_name)
            {
                Ok(_) => {
                    let content = String::from_utf8_lossy(&buffer).to_string();

                    code_files.push(CodeFile {
                        relative_path: entry_name.replace('\\', "/"),
                        content,
                        extension: path_in_archive
                            .extension()
                            .and_then(|s| s.to_str())
                            .unwrap_or("")
                            .to_string(),
                    });
                },
                Err(e) => {
                    // Log warning but don't fail the whole archive if one file is corrupt
                    log::warn!("Failed to extract '{}' from archive: {}", entry_name, e);
                    continue;
                },
            }
        }

        if code_files.is_empty() {
            return Err(FileError::EmptySubmission);
        }

        Ok(Submission {
            metadata: submission_metadata,
            files: code_files,
        })
    }

    /// Recursively scans a directory on the disk.
    fn process_folder(
        root_path: &Path, settings: &DatabaseSettings,
    ) -> Result<Submission, FileError> {
        let directory_name = root_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(FileError::FailedToGetStem)?;

        let submission_metadata =
            SubmissionMetadata::parse(directory_name, &settings.file_name_pattern)
                .map_err(|_| FileError::InvalidPattern)?;
        let mut code_files = Vec::new();

        for entry in WalkDir::new(root_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            // Calculate relative path for database storage
            let relative_path = path.strip_prefix(root_path).unwrap_or(path);

            // Filter 1: Ignore and Whitelist
            if !Self::is_path_allowed(relative_path, settings) {
                continue;
            }

            // Read content
            if let Ok(content_bytes) = std::fs::read(path) {
                let content = String::from_utf8_lossy(&content_bytes).to_string();

                code_files.push(CodeFile {
                    relative_path: relative_path.to_string_lossy().replace('\\', "/"),
                    content,
                    extension: path
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }

        if code_files.is_empty() {
            return Err(FileError::EmptySubmission);
        }

        Ok(Submission {
            metadata: submission_metadata,
            files: code_files,
        })
    }

    /// Helper: Checks if the path is valid based on Blacklist and Whitelist settings.
    fn is_path_allowed(path: &Path, settings: &DatabaseSettings) -> bool {
        // 1. Blacklist Check (Directories)
        // Check if ANY component of the path is in the ignored list.
        for component in path.components() {
            if let Some(component) = component.as_os_str().to_str()
                && settings
                    .ignore_list
                    .ignored_directories
                    .iter()
                    .any(|directory| directory.eq(component))
            {
                return false;
            }
        }

        // 2. Whitelist Check (Extensions)
        if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
            let extension = extension.to_lowercase();
            return settings
                .ignore_list
                .accepted_extensions
                .contains(&extension);
        }

        false
    }
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to create file: {0}")]
    Create(std::io::Error),

    #[error("Failed to open file: {0}")]
    Open(std::io::Error),

    #[error("Failed to read content from file: {0}")]
    Read(std::io::Error),

    #[error("Failed to write content into file: {0}")]
    Write(std::io::Error),
}
