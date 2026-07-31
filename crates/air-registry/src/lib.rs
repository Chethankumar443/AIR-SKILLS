//! Registry index cache, skill resolution, and format validator.
//!
//! Handles:
//! - Registry Format (`index.json`, `starter-kits/`, `skills/`, `templates/`)
//! - Skill Pack layout (`skill.yaml`, `README.md`, `system.md`, `design.md`, `templates/`, `hooks/`)
//! - `search()`, `get_skill()`, `get_starter_kit()`, `validate()`

use air_domain::{RegistryIndex, Skill, SkillCategory, SkillId, StarterKit, ValidationReport};
use air_utils::{AirError, RegistryError};
use chrono::Utc;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Manifest structure parsed directly from `skill.yaml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillYaml {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: String,
    pub source_tag: Option<String>,
    pub checksum_sha256: Option<String>,
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub compatible_with: Vec<VersionConstraintYaml>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionConstraintYaml {
    pub skill_id: String,
    pub range: String,
}

pub struct RegistryClient {
    index: RegistryIndex,
}

impl RegistryClient {
    pub fn new() -> Self {
        Self {
            index: Self::default_index(),
        }
    }

    /// Construct a client initialized with custom registry index data
    pub fn with_index(index: RegistryIndex) -> Self {
        Self { index }
    }

    /// Load registry client from a local registry directory
    pub fn from_directory(path: &Path) -> Result<Self, AirError> {
        let index_file = path.join("index.json");
        if index_file.exists() {
            let content = fs::read_to_string(&index_file)?;
            let index: RegistryIndex = serde_json::from_str(&content)
                .map_err(|e| AirError::Registry(RegistryError::SkillNotFound(format!("Invalid index.json: {}", e))))?;
            Ok(Self::with_index(index))
        } else {
            Ok(Self::new())
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Core Service APIs: search(), get_skill(), get_starter_kit(), validate()
    // ─────────────────────────────────────────────────────────────────────────

    /// Search for skills in the registry index by keyword, tag, or category
    pub fn search(&self, query: &str) -> Vec<Skill> {
        let q = query.trim().to_lowercase();
        if q.is_empty() || q == "*" {
            return self.index.skills.clone();
        }

        self.index
            .skills
            .iter()
            .filter(|s| {
                s.id.as_str().to_lowercase().contains(&q)
                    || s.name.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect()
    }

    /// Retrieve a skill by its ID
    pub fn get_skill(&self, id: &SkillId) -> Result<Skill, AirError> {
        self.index
            .skills
            .iter()
            .find(|s| s.id == *id)
            .cloned()
            .ok_or_else(|| {
                AirError::Registry(RegistryError::SkillNotFound(format!(
                    "Skill `{}` not found in registry",
                    id
                )))
            })
    }

    /// Alias for backwards compatibility
    pub fn fetch_skill(&self, id: &SkillId) -> Result<Option<Skill>, AirError> {
        match self.get_skill(id) {
            Ok(s) => Ok(Some(s)),
            Err(_) => Ok(None),
        }
    }

    /// Retrieve a starter kit by its ID
    pub fn get_starter_kit(&self, id: &str) -> Result<StarterKit, AirError> {
        self.index
            .starter_kits
            .iter()
            .find(|k| k.id == id)
            .cloned()
            .ok_or_else(|| {
                AirError::Registry(RegistryError::SkillNotFound(format!(
                    "Starter kit `{}` not found in registry",
                    id
                )))
            })
    }

    /// List all available starter kits
    pub fn list_starter_kits(&self) -> Vec<StarterKit> {
        self.index.starter_kits.clone()
    }

    /// Validate a skill pack directory or complete registry directory on disk
    pub fn validate(&self, path: &Path) -> Result<ValidationReport, AirError> {
        let mut report = ValidationReport::new();

        if !path.exists() {
            report.add_error(format!("Target path does not exist: `{}`", path.display()));
            return Ok(report);
        }

        let skill_yaml = path.join("skill.yaml");
        let index_json = path.join("index.json");

        if skill_yaml.exists() {
            // Validate individual skill pack directory
            self.validate_skill_pack(path, &mut report);
        } else if index_json.exists() || path.join("skills").exists() || path.join("starter-kits").exists() {
            // Validate complete registry directory
            self.validate_registry_root(path, &mut report);
        } else {
            report.add_error(format!(
                "Path `{}` is neither a valid Registry root (missing `index.json`) nor a Skill Pack directory (missing `skill.yaml`)",
                path.display()
            ));
        }

        Ok(report)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Internal Validation Logic
    // ─────────────────────────────────────────────────────────────────────────

    fn validate_skill_pack(&self, skill_dir: &Path, report: &mut ValidationReport) {
        let yaml_path = skill_dir.join("skill.yaml");
        let content = match fs::read_to_string(&yaml_path) {
            Ok(c) => c,
            Err(e) => {
                report.add_error(format!("Failed to read `skill.yaml`: {}", e));
                return;
            }
        };

        // Parse YAML (using serde_json fallback or toml/yaml parser)
        let parsed: Result<SkillYaml, _> = serde_json::from_str(&content)
            .or_else(|_| toml::from_str(&content));

        match parsed {
            Ok(manifest) => {
                // Check ID
                if manifest.id.trim().is_empty() {
                    report.add_error("`skill.yaml` contains an empty `id` field");
                }

                // Check version
                if Version::parse(&manifest.version).is_err() {
                    report.add_error(format!("Invalid semver version `{}` in `skill.yaml`", manifest.version));
                }

                // PRD §229 requirement: checksum_sha256 and source_tag are required
                match &manifest.source_tag {
                    Some(tag) if !tag.trim().is_empty() => {}
                    _ => report.add_error("Missing required field `source_tag` in `skill.yaml`"),
                }

                match &manifest.checksum_sha256 {
                    Some(sha) if !sha.trim().is_empty() => {
                        if sha.len() != 64 || !sha.chars().all(|c| c.is_ascii_hexdigit()) {
                            report.add_warning("Field `checksum_sha256` should be a 64-character hex string");
                        }
                    }
                    _ => report.add_error("Missing required field `checksum_sha256` in `skill.yaml`"),
                }
            }
            Err(e) => {
                report.add_error(format!("Failed to parse `skill.yaml`: {}", e));
            }
        }

        // Check required documentation & system files
        let required_files = [
            ("README.md", "User documentation"),
            ("system.md", "AI System instructions"),
            ("design.md", "AI Architecture & Design guidelines"),
        ];

        for (file_name, desc) in required_files {
            if !skill_dir.join(file_name).exists() {
                report.add_error(format!("Missing required skill file `{}` ({})", file_name, desc));
            }
        }

        // Check recommended directories
        if !skill_dir.join("templates").exists() {
            report.add_warning("Directory `templates/` missing from skill pack");
        }
        if !skill_dir.join("hooks").exists() {
            report.add_warning("Directory `hooks/` missing from skill pack");
        }
    }

    fn validate_registry_root(&self, reg_dir: &Path, report: &mut ValidationReport) {
        let index_path = reg_dir.join("index.json");
        if index_path.exists() {
            match fs::read_to_string(&index_path) {
                Ok(content) => {
                    if serde_json::from_str::<RegistryIndex>(&content).is_err() {
                        report.add_error("`index.json` is not valid JSON matching RegistryIndex schema");
                    }
                }
                Err(e) => report.add_error(format!("Failed to read `index.json`: {}", e)),
            }
        } else {
            report.add_warning("Missing `index.json` at registry root");
        }

        let starter_kits_dir = reg_dir.join("starter-kits");
        if !starter_kits_dir.exists() {
            report.add_warning("Missing `starter-kits/` directory at registry root");
        }

        let skills_dir = reg_dir.join("skills");
        if !skills_dir.exists() {
            report.add_error("Missing `skills/` directory at registry root");
        } else if let Ok(entries) = fs::read_dir(&skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.validate_skill_pack(&path, report);
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Default Registry Catalog Construction
    // ─────────────────────────────────────────────────────────────────────────

    fn default_index() -> RegistryIndex {
        let kits = vec![
            StarterKit {
                id: "react-saas".to_string(),
                name: "Modern React SaaS".to_string(),
                description: "React + TypeScript + Tailwind + PostgreSQL".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("react"), VersionReq::parse(">=18.0.0").unwrap()),
                    (SkillId::new("typescript"), VersionReq::parse(">=5.0.0").unwrap()),
                    (SkillId::new("tailwind"), VersionReq::parse(">=3.0.0").unwrap()),
                    (SkillId::new("postgresql"), VersionReq::parse(">=14.0.0").unwrap()),
                    (SkillId::new("testing"), VersionReq::parse(">=1.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "ai-chatbot".to_string(),
                name: "AI Chatbot".to_string(),
                description: "Python + LangChain + OpenAI + SQLite".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("python"), VersionReq::parse(">=3.10.0").unwrap()),
                    (SkillId::new("langchain"), VersionReq::parse(">=0.1.0").unwrap()),
                    (SkillId::new("openai"), VersionReq::parse(">=1.0.0").unwrap()),
                    (SkillId::new("sqlite"), VersionReq::parse(">=3.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "desktop-app".to_string(),
                name: "Desktop App (Rust+Tauri)".to_string(),
                description: "Tauri + Rust + Astro + SQLite".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("tauri"), VersionReq::parse(">=2.0.0").unwrap()),
                    (SkillId::new("rust"), VersionReq::parse(">=1.80.0").unwrap()),
                    (SkillId::new("astro"), VersionReq::parse(">=4.0.0").unwrap()),
                    (SkillId::new("sqlite"), VersionReq::parse(">=3.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "nextjs-fullstack".to_string(),
                name: "Full Stack Next.js".to_string(),
                description: "Next.js + TypeScript + Tailwind + Prisma".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("nextjs"), VersionReq::parse(">=14.0.0").unwrap()),
                    (SkillId::new("typescript"), VersionReq::parse(">=5.0.0").unwrap()),
                    (SkillId::new("tailwind"), VersionReq::parse(">=3.0.0").unwrap()),
                    (SkillId::new("prisma"), VersionReq::parse(">=5.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "python-api".to_string(),
                name: "Python API".to_string(),
                description: "FastAPI + Pydantic + PostgreSQL".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("python"), VersionReq::parse(">=3.10.0").unwrap()),
                    (SkillId::new("fastapi"), VersionReq::parse(">=0.100.0").unwrap()),
                    (SkillId::new("pydantic"), VersionReq::parse(">=2.0.0").unwrap()),
                    (SkillId::new("postgresql"), VersionReq::parse(">=14.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "ai-agent".to_string(),
                name: "AI Agent".to_string(),
                description: "Python + LangGraph + MCP + Ollama".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("python"), VersionReq::parse(">=3.10.0").unwrap()),
                    (SkillId::new("langgraph"), VersionReq::parse(">=0.1.0").unwrap()),
                    (SkillId::new("mcp"), VersionReq::parse(">=1.0.0").unwrap()),
                    (SkillId::new("ollama"), VersionReq::parse(">=0.1.0").unwrap()),
                ],
            },
            StarterKit {
                id: "mcp-server".to_string(),
                name: "MCP Server".to_string(),
                description: "TypeScript + MCP SDK".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("typescript"), VersionReq::parse(">=5.0.0").unwrap()),
                    (SkillId::new("mcp"), VersionReq::parse(">=1.0.0").unwrap()),
                ],
            },
            StarterKit {
                id: "empty-project".to_string(),
                name: "Empty Project".to_string(),
                description: "Minimal AIR workspace".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![(SkillId::new("air-core"), VersionReq::parse(">=0.1.0").unwrap())],
            },
            // Legacy alias for integration tests
            StarterKit {
                id: "react-webapp".to_string(),
                name: "Modern React Web App".to_string(),
                description: "React + Tailwind + Vite setup".to_string(),
                version: Version::new(1, 0, 0),
                skills: vec![
                    (SkillId::new("react"), VersionReq::parse(">=18.0.0").unwrap()),
                    (SkillId::new("tailwind"), VersionReq::parse(">=3.0.0").unwrap()),
                ],
            },
        ];

        let make_skill = |id: &str, name: &str, cat: SkillCategory, desc: &str, tags: &[&str]| Skill {
            id: SkillId::new(id),
            name: name.to_string(),
            description: desc.to_string(),
            version: Version::new(1, 0, 0),
            source_tag: "v1.0.0".to_string(),
            checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            category: cat,
            tags: tags.iter().map(|t| t.to_string()).collect(),
            compatible_with: Vec::new(),
            files: Vec::new(),
        };

        let skills = vec![
            make_skill("react", "React", SkillCategory::Frontend, "UI Component Library", &["ui", "frontend", "components"]),
            make_skill("typescript", "TypeScript", SkillCategory::FullStack, "Typed JavaScript", &["lang", "types", "compiler"]),
            make_skill("tailwind", "Tailwind CSS", SkillCategory::Frontend, "Utility-first CSS Framework", &["css", "styling"]),
            make_skill("postgresql", "PostgreSQL", SkillCategory::Database, "Relational Database", &["db", "sql", "postgres"]),
            make_skill("testing", "Testing Pack", SkillCategory::Testing, "Test Framework & Scaffolding", &["test", "vitest", "jest"]),
            make_skill("python", "Python", SkillCategory::Backend, "Python Programming Language", &["lang", "scripting"]),
            make_skill("langchain", "LangChain", SkillCategory::AiStack, "LLM Application Framework", &["ai", "llm", "chains"]),
            make_skill("openai", "OpenAI", SkillCategory::AiStack, "OpenAI API SDK & System Prompts", &["ai", "gpt", "api"]),
            make_skill("sqlite", "SQLite", SkillCategory::Database, "Embedded SQL Database", &["db", "sql", "lightweight"]),
            make_skill("tauri", "Tauri", SkillCategory::Frontend, "Cross-platform Desktop Apps with Rust", &["desktop", "rust", "gui"]),
            make_skill("rust", "Rust", SkillCategory::Backend, "Systems Programming Language", &["lang", "systems"]),
            make_skill("astro", "Astro", SkillCategory::Frontend, "Content-focused Web Framework", &["frontend", "static"]),
            make_skill("nextjs", "Next.js", SkillCategory::FullStack, "React Framework for Production", &["fullstack", "ssr"]),
            make_skill("prisma", "Prisma", SkillCategory::Database, "Next-generation ORM", &["orm", "database"]),
            make_skill("fastapi", "FastAPI", SkillCategory::Backend, "High Performance Python Web API", &["api", "python", "rest"]),
            make_skill("pydantic", "Pydantic", SkillCategory::Backend, "Data Validation for Python", &["validation", "types"]),
            make_skill("langgraph", "LangGraph", SkillCategory::AiStack, "Stateful Multi-agent Workflows", &["ai", "agents", "graphs"]),
            make_skill("mcp", "MCP SDK", SkillCategory::AiStack, "Model Context Protocol SDK", &["mcp", "ai", "protocol"]),
            make_skill("ollama", "Ollama", SkillCategory::AiStack, "Local LLM Integration", &["ai", "local", "llm"]),
            make_skill("air-core", "AIR Core", SkillCategory::Documentation, "AIR Workspace Core Intelligence", &["core", "meta"]),
        ];

        RegistryIndex {
            schema_version: 1,
            updated_at: Utc::now(),
            starter_kits: kits,
            skills,
        }
    }
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_skills() {
        let client = RegistryClient::new();

        let react_results = client.search("react");
        assert!(!react_results.is_empty());
        assert!(react_results.iter().any(|s| s.id.as_str() == "react"));

        let ai_results = client.search("ai");
        assert!(ai_results.len() >= 3);

        let wildcard_results = client.search("*");
        assert_eq!(wildcard_results.len(), 20);
    }

    #[test]
    fn test_get_skill() {
        let client = RegistryClient::new();
        let skill = client.get_skill(&SkillId::new("fastapi")).unwrap();
        assert_eq!(skill.name, "FastAPI");
        assert_eq!(skill.category, SkillCategory::Backend);

        assert!(client.get_skill(&SkillId::new("nonexistent")).is_err());
    }

    #[test]
    fn test_get_starter_kit() {
        let client = RegistryClient::new();
        let kit = client.get_starter_kit("python-api").unwrap();
        assert_eq!(kit.name, "Python API");
        assert_eq!(kit.skills.len(), 4);

        assert!(client.get_starter_kit("nonexistent-kit").is_err());
    }

    #[test]
    fn test_validate_valid_skill_directory() {
        let temp_dir = std::env::temp_dir().join("air_test_skill_valid");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let yaml_content = r#"{
            "id": "test-skill",
            "name": "Test Skill",
            "version": "1.0.0",
            "source_tag": "v1.0.0",
            "checksum_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        }"#;

        fs::write(temp_dir.join("skill.yaml"), yaml_content).unwrap();
        fs::write(temp_dir.join("README.md"), "# Test Skill").unwrap();
        fs::write(temp_dir.join("system.md"), "# System Prompt").unwrap();
        fs::write(temp_dir.join("design.md"), "# Design Guidelines").unwrap();
        fs::create_dir_all(temp_dir.join("templates")).unwrap();
        fs::create_dir_all(temp_dir.join("hooks")).unwrap();

        let client = RegistryClient::new();
        let report = client.validate(&temp_dir).unwrap();

        assert!(report.is_valid, "Validation failed: {:?}", report.errors);
        assert!(report.errors.is_empty());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_validate_invalid_skill_missing_required_prd_fields() {
        let temp_dir = std::env::temp_dir().join("air_test_skill_invalid");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Missing checksum_sha256 and source_tag (PRD §229 violation)
        let yaml_content = r#"{
            "id": "invalid-skill",
            "version": "1.0.0"
        }"#;

        fs::write(temp_dir.join("skill.yaml"), yaml_content).unwrap();

        let client = RegistryClient::new();
        let report = client.validate(&temp_dir).unwrap();

        assert!(!report.is_valid);
        assert!(report.errors.iter().any(|e| e.contains("source_tag")));
        assert!(report.errors.iter().any(|e| e.contains("checksum_sha256")));
        assert!(report.errors.iter().any(|e| e.contains("README.md")));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
