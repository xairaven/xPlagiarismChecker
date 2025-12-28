use crate::models::database::DatabaseError;
use crate::models::submission::SubmissionMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileNamePattern {
    /// Example: "Petrov_Lab1.zip"
    StudentTask { separator: char },

    /// Example: "Lab1_Petrov.zip"
    TaskStudent { separator: char },

    /// Without task (file title = student): "Petrov.zip"
    /// -> (Student="Petrov", Task="Unknown")
    StudentOnly,
}

impl Default for FileNamePattern {
    fn default() -> Self {
        Self::StudentTask { separator: '_' }
    }
}

impl SubmissionMetadata {
    pub fn parse(
        filename: &str, pattern: &FileNamePattern,
    ) -> Result<Self, DatabaseError> {
        match pattern {
            FileNamePattern::StudentTask { separator } => {
                let (student, task) = filename
                    .split_once(*separator)
                    .ok_or(DatabaseError::InvalidPattern(filename.to_string()))?;

                Ok(SubmissionMetadata {
                    student_name: student.to_string(),
                    assignment_title: Some(task.to_string()),
                })
            },
            FileNamePattern::TaskStudent { separator } => {
                let (task, student) = filename
                    .split_once(*separator)
                    .ok_or(DatabaseError::InvalidPattern(filename.to_string()))?;

                Ok(SubmissionMetadata {
                    student_name: student.to_string(),
                    assignment_title: Some(task.to_string()),
                })
            },
            FileNamePattern::StudentOnly => Ok(SubmissionMetadata {
                student_name: filename.to_string(),
                assignment_title: None,
            }),
        }
    }
}
