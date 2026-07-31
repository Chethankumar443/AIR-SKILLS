use air_core::{
    DefaultInstallService, DefaultRepositoryService, InstallRequest, InstallService,
    RepositorySource,
};
use air_domain::{ProgressEvent, SkillCategory, SkillId};
use air_github::GitHubClient;
use air_merge::MergeEngine;
use air_workspace::WorkspaceGenerator;
use semver::Version;
use std::fs;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_phase17_repository_service_download_and_checksum() {
    let repo_service = DefaultRepositoryService::new(None);
    let skill = air_domain::Skill {
        id: SkillId::new("react"),
        name: "React".to_string(),
        description: "React UI".to_string(),
        version: Version::new(1, 0, 0),
        source_tag: "v1.0.0".to_string(),
        checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        category: SkillCategory::Frontend,
        tags: Vec::new(),
        compatible_with: Vec::new(),
        files: Vec::new(),
    };

    let bytes = repo_service.fetch_skill_archive(&skill).await.unwrap();
    assert!(!bytes.is_empty());
}

#[tokio::test]
async fn test_phase18_installer_engine_full_pipeline_with_events() {
    let temp_dir = std::env::temp_dir().join("air_phase18_pipeline");
    let _ = fs::remove_dir_all(&temp_dir);

    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: Some("python-api".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };

    let plan = install_service.create_plan(&request).unwrap();
    let (tx, mut rx) = mpsc::channel(32);

    let result = install_service.install_with_progress(&plan, Some(tx)).await.unwrap();
    assert!(result.success);

    let mut events = Vec::new();
    while let Ok(event) = rx.try_recv() {
        events.push(event);
    }

    assert!(!events.is_empty());
    assert!(events.iter().any(|e| matches!(e, ProgressEvent::Initializing { .. })));
    assert!(events.iter().any(|e| matches!(e, ProgressEvent::ResolvingDependencies { .. })));
    assert!(events.iter().any(|e| matches!(e, ProgressEvent::Completed { .. })));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_phase19_workspace_generator_deterministic_output() {
    let temp_dir = std::env::temp_dir().join("air_phase19_wsp");
    let _ = fs::remove_dir_all(&temp_dir);

    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: Some("react-saas".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };

    let plan = install_service.create_plan(&request).unwrap();
    let result = install_service.install(&plan).unwrap();
    assert!(result.success);

    // Verify all deterministic files exist
    assert!(temp_dir.join(".air/lock.json").exists());
    assert!(temp_dir.join(".air/config.json").exists());
    assert!(temp_dir.join(".air/merge-audit.json").exists());
    assert!(temp_dir.join("system_instructions.md").exists());
    assert!(temp_dir.join("design.md").exists());
    assert!(temp_dir.join("README.md").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_phase20_merge_engine_deduplication_and_audit() {
    let merge_engine = MergeEngine::new(false);

    let doc1 = "## Guidelines\n- Follow PEP 8\n- Write unit tests\n";
    let doc2 = "## Guidelines\n- Follow PEP 8\n- Add docstrings\n";

    let (merged, audit) = merge_engine
        .merge_markdown_files("Python Guidelines", &[("python-base", doc1), ("fastapi-ext", doc2)])
        .unwrap();

    assert!(merged.contains("# Python Guidelines"));
    assert!(merged.contains("<!-- Source: python-base -->"));
    assert!(merged.contains("<!-- Source: fastapi-ext -->"));
    assert!(merged.contains("Add docstrings"));
    assert_eq!(audit.total_sections_merged, 1);
}
