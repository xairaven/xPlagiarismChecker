use crate::models::submission::Submission;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Database {
    pub file_path: Option<PathBuf>,

    pub is_dirty: bool,
    pub meta: Meta,
    pub submissions: Vec<Submission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: i16,
}

impl Database {
    pub fn new(name: String, description: Option<String>, path: Option<PathBuf>) -> Self {
        Self {
            file_path: path,
            is_dirty: true,
            meta: Meta {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                description,
                version: 1,
            },
            submissions: vec![],
        }
    }
}
