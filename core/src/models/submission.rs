use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    // Unique identifier for the student
    pub student_id: String,
    pub files: Vec<CodeFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub relative_path: String,
    pub content: String,
    pub extension: String,
}
