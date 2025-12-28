use crate::errors::LibError;
use crate::io::IoError;
use crate::models::pattern::FileNamePattern;
use crate::models::submission::{CodeFile, Submission, SubmissionMetadata};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use zip::write::SimpleFileOptions;

pub const FILE_EXTENSION: &str = "xai";
pub const META_FILE_NAME: &str = "meta.json";
pub const SETTINGS_FILE_NAME: &str = "settings.json";
pub const SUBMISSIONS_DIR: &str = "submissions";

#[derive(Debug)]
pub struct Database {
    pub file_path: PathBuf,

    pub is_dirty: bool,
    pub meta: DatabaseMetadata,
    pub settings: DatabaseSettings,
    pub submissions: Vec<Submission>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub file_name_pattern: FileNamePattern,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseMetadata {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: i16,
}

impl DatabaseMetadata {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            version: 1,
        }
    }
}

impl Database {
    pub fn new(
        name: String, description: Option<String>, settings: DatabaseSettings,
        path: PathBuf,
    ) -> Self {
        Self {
            file_path: path,
            is_dirty: true,
            meta: DatabaseMetadata::new(name, description),
            settings,
            submissions: vec![],
        }
    }

    pub fn save(&mut self) -> Result<(), LibError> {
        let file = std::fs::File::create(&self.file_path).map_err(IoError::Create)?;
        let mut zip = zip::ZipWriter::new(file);

        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);

        // Metadata file
        zip.start_file(META_FILE_NAME, options)
            .map_err(LibError::Zip)?;
        let meta_json =
            serde_json::to_string_pretty(&self.meta).map_err(LibError::Json)?;
        zip.write_all(meta_json.as_bytes())
            .map_err(IoError::Write)?;

        // Settings file
        zip.start_file(SETTINGS_FILE_NAME, options)
            .map_err(LibError::Zip)?;
        let settings_json =
            serde_json::to_string_pretty(&self.settings).map_err(LibError::Json)?;
        zip.write_all(settings_json.as_bytes())
            .map_err(IoError::Write)?;

        // Submissions
        for submission in &self.submissions {
            let mut directory_path =
                format!("{}/{}", SUBMISSIONS_DIR, submission.metadata.student_name);
            if let Some(title) = &submission.metadata.assignment_title {
                directory_path.push_str(&format!("/{}", title));
            }

            for code_file in &submission.files {
                let file_path = format!("{}/{}", directory_path, code_file.relative_path);
                zip.start_file(file_path, options).map_err(LibError::Zip)?;
                zip.write_all(code_file.content.as_bytes())
                    .map_err(IoError::Write)?;
            }
        }

        zip.finish().map_err(LibError::Zip)?;

        self.is_dirty = false;

        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, LibError> {
        let file = std::fs::File::open(path).map_err(IoError::Open)?;
        let mut archive = zip::ZipArchive::new(file).map_err(LibError::Zip)?;

        // Reading Metadata
        let meta: DatabaseMetadata = {
            let mut file = archive
                .by_name(META_FILE_NAME)
                .map_err(|_| DatabaseError::MissingMetadata)?;
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(IoError::Read)?;
            serde_json::from_str(&content).map_err(LibError::Json)?
        };

        // Reading Settings
        let settings: DatabaseSettings = {
            match archive.by_name(SETTINGS_FILE_NAME) {
                Ok(mut file) => {
                    let mut content = String::new();
                    file.read_to_string(&mut content).map_err(IoError::Read)?;
                    serde_json::from_str(&content).map_err(LibError::Json)?
                },
                Err(_) => DatabaseSettings::default(),
            }
        };

        // Reading Submissions
        // Grouping files by (student, assignment)
        let mut grouped_files: HashMap<(String, Option<String>), Vec<CodeFile>> =
            HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(LibError::Zip)?;

            // Skipping directories
            if file.is_dir() {
                continue;
            }

            // Parsing path: submissions/Ivanov/Lab1/src/main.rs
            let path_str = file.name().to_string();
            let parts: Vec<&str> = path_str.split('/').collect();

            // Ignore files not following the pattern
            if parts.len() <= 4
                || (parts.len() <= 3
                    && matches!(settings.file_name_pattern, FileNamePattern::StudentOnly))
            {
                continue;
            }

            // Extracting student, assignment (if applicable), and relative path based on pattern
            let (student, assignment, relative_path) = match &settings.file_name_pattern {
                FileNamePattern::StudentTask { .. } => {
                    match &parts[..] {
                        [SUBMISSIONS_DIR, student, assignment, rest @ ..] => (
                            student.to_string(),
                            Some(assignment.to_string()),
                            rest.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                        ),
                        _ => continue, // Files outside submissions directory are ignored
                    }
                },
                FileNamePattern::TaskStudent { .. } => {
                    match &parts[..] {
                        [SUBMISSIONS_DIR, assignment, student, rest @ ..] => (
                            student.to_string(),
                            Some(assignment.to_string()),
                            rest.iter().map(|s| s.to_string()).collect(),
                        ),
                        _ => continue, // Files outside submissions directory are ignored
                    }
                },
                FileNamePattern::StudentOnly => {
                    match &parts[..] {
                        [SUBMISSIONS_DIR, student, rest @ ..] => (
                            student.to_string(),
                            None,
                            rest.iter().map(|s| s.to_string()).collect(),
                        ),
                        _ => continue, // Files outside submissions directory are ignored
                    }
                },
            };
            let relative_path = relative_path.join("/");

            // Reading content (safe for UTF-8)
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(IoError::Read)?;
            let content = match String::from_utf8(buffer) {
                Ok(content) => content,
                Err(error) => {
                    log::warn!(
                        "File '{}' contains invalid UTF-8 and will be skipped. Error: {}",
                        path_str,
                        error
                    );
                    continue;
                },
            };

            let code_file = CodeFile {
                relative_path,
                content,
                extension: Path::new(&path_str)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string(),
            };

            grouped_files
                .entry((student, assignment))
                .or_default()
                .push(code_file);
        }

        // Converting grouped files into submissions
        let submissions: Vec<Submission> = grouped_files
            .into_iter()
            .map(|((student_name, assignment_title), files)| Submission {
                metadata: SubmissionMetadata {
                    student_name,
                    assignment_title,
                },
                files,
            })
            .collect();

        Ok(Self {
            file_path: path.to_path_buf(),
            is_dirty: false,
            meta,
            settings,
            submissions,
        })
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Filename does not match the expected pattern: {0}")]
    InvalidPattern(String),

    #[error("Database is missing required metadata.")]
    MissingMetadata,
}
