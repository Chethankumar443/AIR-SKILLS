//! Workspace generator responsible for creating `.air/`, `config.json`, `lock.json`, `merge-audit.json`, and writing merged markdown intelligence files.

use air_domain::{InstallPlan, Manifest, StarterKit, WorkspaceResult};
use air_merge::MergeEngine;
use air_utils::AirError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Local workspace configuration format stored in `.air/config.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub schema_version: u32,
    pub workspace_name: String,
    pub created_at: String,
    pub installed_skill_ids: Vec<String>,
}

pub struct WorkspaceGenerator;

impl WorkspaceGenerator {
    /// Generate a deterministic AIR workspace layout at the specified target directory
    pub fn generate(plan: &InstallPlan) -> Result<WorkspaceResult, AirError> {
        let root = Path::new(&plan.target_directory);
        let air_dir = root.join(".air");
        let templates_dir = air_dir.join("templates");

        // 1. Create directory structure
        fs::create_dir_all(&air_dir)?;
        fs::create_dir_all(&templates_dir)?;

        let merge_engine = MergeEngine::new(false);
        let now = Utc::now();

        // 2. Prepare merged documents from resolved skills
        let system_docs: Vec<(&str, &str)> = plan
            .resolved_skills
            .iter()
            .map(|s| (s.id.as_str(), default_system_prompt_for_skill(&s.id.as_str())))
            .collect();

        let design_docs: Vec<(&str, &str)> = plan
            .resolved_skills
            .iter()
            .map(|s| (s.id.as_str(), default_design_rules_for_skill(&s.id.as_str())))
            .collect();

        let readme_docs: Vec<(&str, &str)> = plan
            .resolved_skills
            .iter()
            .map(|s| (s.id.as_str(), default_readme_for_skill(&s.id.as_str())))
            .collect();

        // 3. Perform deterministic merge
        let (system_instructions_content, system_audit) =
            merge_engine.merge_markdown_files("AIR System Instructions", &system_docs)?;

        let (design_content, _design_audit) =
            merge_engine.merge_markdown_files("AIR Architecture & Design Guidelines", &design_docs)?;

        let (readme_content, _readme_audit) =
            merge_engine.merge_markdown_files("Project Overview & Installed Skills", &readme_docs)?;

        // 4. Write merged intelligence files
        fs::write(root.join("system_instructions.md"), system_instructions_content)?;
        fs::write(root.join("design.md"), design_content)?;

        // Write README.md if it doesn't already exist or as standard output
        if !root.join("README.md").exists() {
            fs::write(root.join("README.md"), readme_content)?;
        }

        // 5. Write `.air/lock.json`
        let lock_manifest = Manifest {
            schema_version: 1,
            installed_skills: plan.resolved_skills.clone(),
            starter_kit: plan.starter_kit_id.as_ref().map(|id| StarterKit {
                id: id.clone(),
                name: id.clone(),
                description: format!("Starter kit {}", id),
                version: semver::Version::new(1, 0, 0),
                skills: Vec::new(),
            }),
            created_at: now,
            updated_at: now,
        };

        let lock_json = serde_json::to_string_pretty(&lock_manifest)
            .map_err(|e| AirError::Storage(format!("Failed to serialize lock.json: {}", e)))?;
        fs::write(air_dir.join("lock.json"), lock_json)?;

        // 6. Write `.air/config.json`
        let config = WorkspaceConfig {
            schema_version: 1,
            workspace_name: root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "workspace".to_string()),
            created_at: now.to_rfc3339(),
            installed_skill_ids: plan.resolved_skills.iter().map(|s| s.id.to_string()).collect(),
        };

        let config_json = serde_json::to_string_pretty(&config)
            .map_err(|e| AirError::Config(format!("Failed to serialize config.json: {}", e)))?;
        fs::write(air_dir.join("config.json"), config_json)?;

        // 7. Write `.air/merge-audit.json`
        let audit_json = serde_json::to_string_pretty(&system_audit)
            .map_err(|e| AirError::Storage(format!("Failed to serialize merge-audit.json: {}", e)))?;
        fs::write(air_dir.join("merge-audit.json"), audit_json)?;

        Ok(WorkspaceResult {
            success: true,
            workspace_path: plan.target_directory.clone(),
            installed_count: plan.resolved_skills.len(),
            audit_file_path: Some(format!("{}/.air/merge-audit.json", plan.target_directory)),
        })
    }
}

fn default_system_prompt_for_skill(skill_id: &str) -> &'static str {
    match skill_id {
        "react" => "## React Guidelines\n- Prefer functional components and hooks.\n- Keep components modular and single-responsibility.\n",
        "typescript" => "## TypeScript Standards\n- Enable strict mode in tsconfig.json.\n- Avoid using `any`; define explicit types for all function inputs and outputs.\n",
        "tailwind" => "## Tailwind CSS Rules\n- Use utility classes consistently.\n- Prefer HSL color tokens for dark/light theme support.\n",
        "fastapi" => "## FastAPI Standards\n- Use async def for endpoint handlers.\n- Leverage Pydantic models for request validation.\n",
        "python" => "## Python Guidelines\n- Follow PEP 8 style standards.\n- Enforce type hints on all function signatures.\n",
        _ => "## General AI Development Rules\n- Write modular, readable, self-documenting code.\n- Include automated unit tests for core domain logic.\n",
    }
}

fn default_design_rules_for_skill(skill_id: &str) -> &'static str {
    match skill_id {
        "react" => "## Component Architecture\n- Use atomic design principles for UI components.\n",
        "typescript" => "## Type Safety Architecture\n- Define shared domain interfaces in `types/`.\n",
        "python" | "fastapi" => "## Backend Architecture\n- Separate API controllers from core domain services.\n",
        _ => "## System Architecture\n- Keep domain logic decoupled from framework entry points.\n",
    }
}

fn default_readme_for_skill(skill_id: &str) -> &'static str {
    match skill_id {
        "react" => "## Installed Skill: React\nReact UI stack installed with Tailwind & TypeScript support.\n",
        "python" => "## Installed Skill: Python\nPython backend stack configured with type validation.\n",
        _ => "## Installed Skill\nInstalled skill pack providing AI project intelligence.\n",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use air_domain::{Skill, SkillCategory, SkillId};
    use semver::Version;

    #[test]
    fn test_workspace_generation_creates_required_files() {
        let temp_dir = std::env::temp_dir().join("air_test_wsp_gen");
        let _ = fs::remove_dir_all(&temp_dir);

        let plan = InstallPlan {
            starter_kit_id: Some("react-saas".to_string()),
            resolved_skills: vec![
                Skill {
                    id: SkillId::new("react"),
                    name: "React".to_string(),
                    description: "React".to_string(),
                    version: Version::new(1, 0, 0),
                    source_tag: "v1.0.0".to_string(),
                    checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
                    category: SkillCategory::Frontend,
                    tags: Vec::new(),
                    compatible_with: Vec::new(),
                    files: Vec::new(),
                },
                Skill {
                    id: SkillId::new("typescript"),
                    name: "TypeScript".to_string(),
                    description: "TypeScript".to_string(),
                    version: Version::new(1, 0, 0),
                    source_tag: "v1.0.0".to_string(),
                    checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
                    category: SkillCategory::FullStack,
                    tags: Vec::new(),
                    compatible_with: Vec::new(),
                    files: Vec::new(),
                },
            ],
            conflicts: Vec::new(),
            target_directory: temp_dir.to_string_lossy().to_string(),
        };

        let result = WorkspaceGenerator::generate(&plan).unwrap();
        assert!(result.success);
        assert_eq!(result.installed_count, 2);

        // Verify generated files
        assert!(temp_dir.join(".air/lock.json").exists());
        assert!(temp_dir.join(".air/config.json").exists());
        assert!(temp_dir.join(".air/merge-audit.json").exists());
        assert!(temp_dir.join("system_instructions.md").exists());
        assert!(temp_dir.join("design.md").exists());
        assert!(temp_dir.join("README.md").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
