use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

/// Unique identifier for a skill (e.g. "astro", "react-webapp")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId(pub String);

impl SkillId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SkillId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for SkillId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for SkillId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for SkillId {
    fn from(s: String) -> Self {
        Self(s)
    }
}


/// Skill category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillCategory {
    Frontend,
    Backend,
    Database,
    AiStack,
    Testing,
    Documentation,
    Deployment,
    FullStack,
}

/// Semver constraint pair for inter-skill compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraint {
    pub skill_id: SkillId,
    pub range: VersionReq,
}

/// Metadata representation of a file within a Skill Pack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub path: String,
    pub checksum_sha256: String,
    pub is_template: bool,
}

/// Domain model representing an official Skill Pack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub version: Version,
    pub source_tag: String,
    pub checksum_sha256: String,
    pub category: SkillCategory,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub compatible_with: Vec<VersionConstraint>,
    #[serde(default)]
    pub files: Vec<SkillFile>,
}

/// Validation report generated when auditing a registry or skill directory format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, err: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(err.into());
    }

    pub fn add_warning(&mut self, warn: impl Into<String>) {
        self.warnings.push(warn.into());
    }
}

/// Index structure representing `registry/index.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryIndex {
    pub schema_version: u32,
    pub updated_at: DateTime<Utc>,
    pub starter_kits: Vec<StarterKit>,
    pub skills: Vec<Skill>,
}

/// A curated starter kit of mutually compatible skill packs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterKit {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Version,
    pub skills: Vec<(SkillId, VersionReq)>,
}

/// Target project workspace state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub root_path: String,
    pub name: String,
    pub is_initialized: bool,
    pub schema_version: u32,
    pub installed_skills: Vec<Skill>,
}

/// Manifest saved into `.air/lock.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub schema_version: u32,
    pub installed_skills: Vec<Skill>,
    pub starter_kit: Option<StarterKit>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A dependency specification between skill packs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub source_skill: SkillId,
    pub target_skill: SkillId,
    pub required_range: VersionReq,
}

/// Conflict resolution strategy during markdown merging
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    AppendWithAttribution,
    FailStrict,
}

/// Merge conflict descriptor populated pre-install by dry-run merge check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub section_heading: String,
    pub contributing_skills: Vec<SkillId>,
    pub resolution: ConflictResolution,
}

/// Resolved installation plan ready for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallPlan {
    pub starter_kit_id: Option<String>,
    pub resolved_skills: Vec<Skill>,
    pub conflicts: Vec<Conflict>,
    pub target_directory: String,
}
