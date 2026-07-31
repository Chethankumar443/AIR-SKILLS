use serde::{Deserialize, Serialize};

/// Validation result produced when verifying a skill pack or workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn with_errors(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }
}

/// Final result returned after completing workspace generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceResult {
    pub success: bool,
    pub workspace_path: String,
    pub installed_count: usize,
    pub audit_file_path: Option<String>,
}
