use thiserror::Error;

/// Unified error enum across all AIR.SKILLS crates with stable error codes
#[derive(Error, Debug)]
pub enum AirError {
    #[error("[AIR-INS-001] Install Error: {0}")]
    Install(#[from] InstallError),

    #[error("[AIR-VAL-002] Validation Error: {0}")]
    Validation(#[from] ValidationError),

    #[error("[AIR-GH-003] Download / GitHub Error: {0}")]
    Download(#[from] DownloadError),

    #[error("[AIR-MRG-004] Merge Engine Error: {0}")]
    Merge(#[from] MergeError),

    #[error("[AIR-REG-005] Registry Error: {0}")]
    Registry(#[from] RegistryError),

    #[error("[AIR-WSP-006] Workspace Error: {0}")]
    Workspace(#[from] WorkspaceError),

    #[error("[AIR-CFG-007] Config Error: {0}")]
    Config(String),

    #[error("[AIR-STO-008] Storage Error: {0}")]
    Storage(String),

    #[error("[AIR-SYS-099] System I/O Error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Target directory is not empty and --force was not specified: {0}")]
    DirectoryNotEmpty(String),

    #[error("Incompatible skill pack combination detected: {0}")]
    IncompatibleSkills(String),

    #[error("Installation canceled by user")]
    UserCanceled,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Skill pack manifest `skill.yaml` is invalid: {0}")]
    InvalidManifest(String),

    #[error("Missing required source tag or SHA-256 checksum")]
    MissingSourceTagOrChecksum,
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("SHA-256 Checksum mismatch! Expected: {expected}, Got: {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("GitHub API rate limit exceeded. Please configure a GitHub PAT.")]
    RateLimitExceeded,

    #[error("Failed to fetch release archive from GitHub: {0}")]
    NetworkFailed(String),
}

#[derive(Error, Debug)]
pub enum MergeError {
    #[error("Strict mode enabled: Unresolved merge conflict in heading `{heading}` between skills: {skills:?}")]
    StrictConflict { heading: String, skills: Vec<String> },

    #[error("Failed to parse markdown section: {0}")]
    ParseFailed(String),
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Skill pack `{0}` not found in official registry index")]
    SkillNotFound(String),

    #[error("Unresolvable semver version requirement: {0}")]
    VersionUnresolvable(String),
}

#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Security Exception: Path traversal / Zip-slip attempt detected: `{0}`")]
    ZipSlipAttempt(String),

    #[error("Workspace already initialized at `{0}`")]
    AlreadyInitialized(String),
}
