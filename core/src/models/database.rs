use crate::algorithm::file::FileNamePattern;
use crate::errors::LibError;
use crate::io::IoError;
use crate::models::submission::Submission;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
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
        let file = std::fs::File::create(&self.file_path).map_err(IoError::CreateFile)?;
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
}
