use crate::errors::WorkspaceError;
use std::path::{Component, Path, PathBuf};

/// Validates that a target path remains inside a specified destination directory (Zip-Slip guard)
pub fn validate_safe_path(base_dir: &Path, target_rel_path: &Path) -> Result<PathBuf, WorkspaceError> {
    if target_rel_path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(WorkspaceError::ZipSlipAttempt(
            target_rel_path.to_string_lossy().into_owned(),
        ));
    }
    Ok(base_dir.join(target_rel_path))
}

