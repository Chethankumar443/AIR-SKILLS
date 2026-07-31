use crate::services::WorkspaceService;
use air_config::ConfigMigrator;
use air_domain::{InstallPlan, ValidationResult, WorkspaceResult};
use air_utils::{validate_safe_path, AirError};
use air_workspace::WorkspaceGenerator;
use std::fs;
use std::path::Path;

/// Concrete implementation of WorkspaceService responsible for workspace creation and doctor health checks
#[derive(Default)]
pub struct DefaultWorkspaceService;

impl DefaultWorkspaceService {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceService for DefaultWorkspaceService {
    fn generate(&self, plan: &InstallPlan) -> Result<WorkspaceResult, AirError> {
        let base_path = Path::new(&plan.target_directory);
        let anchor = if base_path.is_absolute() {
            base_path.parent().unwrap_or(base_path)
        } else {
            Path::new(".")
        };
        let safe_target = validate_safe_path(base_path, anchor)?;

        // 1. Ensure target directory exists
        if let Err(_e) = fs::create_dir_all(&safe_target) {
            // Safe fallback if target is current or relative directory
            fs::create_dir_all(&base_path)?;
        }

        // 2. Delegate to air-workspace generator
        WorkspaceGenerator::generate(plan)
    }

    fn doctor(&self, target_directory: &str) -> Result<ValidationResult, AirError> {
        let dir = Path::new(target_directory);
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if !dir.exists() {
            errors.push(format!("Target directory `{}` does not exist.", target_directory));
            return Ok(ValidationResult {
                is_valid: false,
                errors,
                warnings,
            });
        }

        let air_dir = dir.join(".air");
        if !air_dir.exists() {
            warnings.push("Workspace is not yet initialized with `.air/` configuration.".to_string());
        } else {
            let config_path = air_dir.join("config.json");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Err(e) = ConfigMigrator::migrate_if_needed(&content) {
                        errors.push(format!("Config schema error: {}", e));
                    }
                }
            } else {
                warnings.push("Missing `.air/config.json` configuration file.".to_string());
            }
        }

        Ok(ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        })
    }
}
