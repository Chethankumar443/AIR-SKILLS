use air_utils::AirError;
use std::fs;
use std::path::Path;

/// Service trait for safely uninstalling AIR workspace metadata
pub trait UninstallService: Send + Sync {
    fn uninstall(&self, target_directory: &str, purge_generated: bool) -> Result<UninstallResult, AirError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UninstallResult {
    pub success: bool,
    pub removed_paths: Vec<String>,
    pub message: String,
}

#[derive(Default)]
pub struct DefaultUninstallService;

impl DefaultUninstallService {
    pub fn new() -> Self {
        Self
    }
}

impl UninstallService for DefaultUninstallService {
    fn uninstall(&self, target_directory: &str, purge_generated: bool) -> Result<UninstallResult, AirError> {
        let base_path = Path::new(target_directory);
        let mut removed_paths = Vec::new();

        let air_dir = base_path.join(".air");
        if air_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&air_dir) {
                return Err(AirError::Io(e));
            }
            removed_paths.push(".air/".to_string());
        }

        if purge_generated {
            let system_doc = base_path.join("system_instructions.md");
            if system_doc.exists() {
                let _ = fs::remove_file(&system_doc);
                removed_paths.push("system_instructions.md".to_string());
            }

            let design_doc = base_path.join("design.md");
            if design_doc.exists() {
                let _ = fs::remove_file(&design_doc);
                removed_paths.push("design.md".to_string());
            }
        }

        let message = if removed_paths.is_empty() {
            "No AIR metadata was found in target directory.".to_string()
        } else {
            format!("Successfully uninstalled AIR metadata: {}", removed_paths.join(", "))
        };

        Ok(UninstallResult {
            success: true,
            removed_paths,
            message,
        })
    }
}
