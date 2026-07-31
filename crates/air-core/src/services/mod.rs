pub mod install_service;
pub mod merge_service;
pub mod registry_service;
pub mod repository_service;
pub mod uninstall_service;
pub mod workspace_service;

use air_domain::{InstallPlan, Skill, SkillId, ValidationResult, WorkspaceResult};
use air_utils::AirError;

/// Request input for an installation operation
#[derive(Debug, Clone)]
pub struct InstallRequest {
    pub target_directory: String,
    pub starter_kit_id: Option<String>,
    pub custom_skills: Vec<SkillId>,
    pub strict_merge: bool,
}

/// Service trait for managing installations
pub trait InstallService: Send + Sync {
    fn create_plan(&self, request: &InstallRequest) -> Result<InstallPlan, AirError>;
    fn install(&self, plan: &InstallPlan) -> Result<WorkspaceResult, AirError>;
}

/// Service trait for fetching skill metadata from repositories
pub trait RepositoryService: Send + Sync {
    fn fetch_skill(&self, id: &SkillId) -> Result<Option<Skill>, AirError>;
}

/// Service trait for performing deterministic merges
pub trait MergeService: Send + Sync {
    fn merge(&self, skills: &[Skill]) -> Result<ValidationResult, AirError>;
}

/// Service trait for workspace creation & health inspection
pub trait WorkspaceService: Send + Sync {
    fn generate(&self, plan: &InstallPlan) -> Result<WorkspaceResult, AirError>;
    fn doctor(&self, target_directory: &str) -> Result<ValidationResult, AirError>;
}

pub use install_service::DefaultInstallService;
pub use merge_service::DefaultMergeService;
pub use registry_service::DefaultRegistryService;
pub use repository_service::DefaultRepositoryService;
pub use uninstall_service::{DefaultUninstallService, UninstallResult, UninstallService};
pub use workspace_service::DefaultWorkspaceService;
