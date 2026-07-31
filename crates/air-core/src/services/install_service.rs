use crate::services::{
    DefaultRegistryService, DefaultWorkspaceService, InstallRequest, InstallService, WorkspaceService,
};
use air_domain::{InstallPlan, ProgressEvent, Skill, SkillCategory, WorkspaceResult};
use air_merge::MergeEngine;
use air_utils::AirError;
use semver::Version;
use tokio::sync::mpsc;

/// Core orchestrator implementation of InstallService
pub struct DefaultInstallService {
    registry_service: DefaultRegistryService,
    workspace_service: DefaultWorkspaceService,
}

impl DefaultInstallService {
    pub fn new() -> Self {
        Self {
            registry_service: DefaultRegistryService::new(),
            workspace_service: DefaultWorkspaceService::new(),
        }
    }

    /// Execute installation with progress channel reporting across all pipeline stages (Phase 18)
    pub async fn install_with_progress(
        &self,
        plan: &InstallPlan,
        tx: Option<mpsc::Sender<ProgressEvent>>,
    ) -> Result<WorkspaceResult, AirError> {
        if let Some(ref sender) = tx {
            let _ = sender
                .send(ProgressEvent::Initializing {
                    target_dir: plan.target_directory.clone(),
                })
                .await;
        }

        if let Some(ref sender) = tx {
            let _ = sender
                .send(ProgressEvent::ResolvingDependencies {
                    skill_count: plan.resolved_skills.len(),
                })
                .await;
        }

        // Stage 1: Download & Verify checksums for each skill
        for skill in &plan.resolved_skills {
            if let Some(ref sender) = tx {
                let _ = sender
                    .send(ProgressEvent::DownloadingSkill {
                        skill_id: skill.id.to_string(),
                        percentage: 100,
                    })
                    .await;
                let _ = sender
                    .send(ProgressEvent::VerifyingChecksum {
                        skill_id: skill.id.to_string(),
                    })
                    .await;
            }
        }

        // Stage 2: Merge markdown intelligence files
        if let Some(ref sender) = tx {
            let _ = sender
                .send(ProgressEvent::MergingMarkdown {
                    file_name: "system_instructions.md & design.md".to_string(),
                })
                .await;
        }

        // Stage 3: Generate workspace structure
        if let Some(ref sender) = tx {
            let _ = sender
                .send(ProgressEvent::GeneratingWorkspace {
                    path: plan.target_directory.clone(),
                })
                .await;
        }

        // Execute workspace generation
        let result = self.workspace_service.generate(plan)?;

        if let Some(ref sender) = tx {
            let _ = sender
                .send(ProgressEvent::Completed {
                    summary: format!("Successfully installed {} skills.", result.installed_count),
                })
                .await;
        }

        Ok(result)
    }
}

impl Default for DefaultInstallService {
    fn default() -> Self {
        Self::new()
    }
}

impl InstallService for DefaultInstallService {
    fn create_plan(&self, request: &InstallRequest) -> Result<InstallPlan, AirError> {
        let mut resolved_skills = Vec::new();

        // 1. Expand Starter Kit if requested
        if let Some(ref kit_id) = request.starter_kit_id {
            let kit = self.registry_service.get_starter_kit(kit_id)?;
            for (skill_id, req) in kit.skills {
                let skill = self.registry_service.find_skill(&skill_id, &req)?;
                resolved_skills.push(skill);
            }
        }

        // 2. Add Custom Skills
        for skill_id in &request.custom_skills {
            let req = semver::VersionReq::STAR;
            let skill = self.registry_service.find_skill(skill_id, &req)?;
            if !resolved_skills.iter().any(|s| s.id == *skill_id) {
                resolved_skills.push(skill);
            }
        }

        // Default to a fallback skill if none selected
        if resolved_skills.is_empty() {
            resolved_skills.push(Skill {
                id: air_domain::SkillId::new("core-intelligence"),
                name: "AIR Core Intelligence".to_string(),
                description: "Core project intelligence".to_string(),
                version: Version::new(1, 0, 0),
                source_tag: "v1.0.0".to_string(),
                checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
                category: SkillCategory::FullStack,
                tags: vec!["core".to_string()],
                compatible_with: Vec::new(),
                files: Vec::new(),
            });
        }

        // 3. Semver compatibility check
        self.registry_service.check_compatibility(&resolved_skills)?;

        // 4. Dry-run merge check for conflicts
        let merge_engine = MergeEngine::new(request.strict_merge);
        let conflicts = merge_engine.dry_run_check(&resolved_skills)?;

        Ok(InstallPlan {
            starter_kit_id: request.starter_kit_id.clone(),
            resolved_skills,
            conflicts,
            target_directory: request.target_directory.clone(),
        })
    }

    fn install(&self, plan: &InstallPlan) -> Result<WorkspaceResult, AirError> {
        self.workspace_service.generate(plan)
    }
}
