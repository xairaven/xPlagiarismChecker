use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub metadata: SubmissionMetadata,
    pub files: Vec<CodeFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionMetadata {
    // For example: "John-Doe"
    pub student_name: String,

    // For example: "Lab7"
    pub assignment_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub relative_path: String,
    pub content: String,
    pub extension: String,
}
