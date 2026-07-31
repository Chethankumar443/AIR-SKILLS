use air_core::{
    DefaultInstallService, DefaultRegistryService, DefaultWorkspaceService, InstallRequest,
    InstallService, WorkspaceService,
};
use air_domain::SkillId;
use air_storage::{InMemoryStorageRepository, StorageRepository};
use std::fs;

#[test]
fn test_phase21_cli_add_command_flow() {
    let temp_dir = std::env::temp_dir().join("air_phase21_add");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: None,
        custom_skills: vec![SkillId::new("react"), SkillId::new("tailwind")],
        strict_merge: false,
    };

    let plan = install_service.create_plan(&request).unwrap();
    let result = install_service.install(&plan).unwrap();

    assert!(result.success);
    assert_eq!(result.installed_count, 2);
    assert!(temp_dir.join(".air/lock.json").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_phase21_cli_search_command_flow() {
    let registry_service = DefaultRegistryService::new();

    let react_results = registry_service.search("react");
    assert!(!react_results.is_empty());
    assert_eq!(react_results[0].id.as_str(), "react");

    let all_skills = registry_service.search("*");
    assert!(all_skills.len() >= 10);
}

#[test]
fn test_phase21_cli_doctor_command_flow() {
    let temp_dir = std::env::temp_dir().join("air_phase21_doctor");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let workspace_service = DefaultWorkspaceService::new();

    // Before init -> warnings
    let initial_doc = workspace_service.doctor(&temp_dir.to_string_lossy()).unwrap();
    assert!(initial_doc.is_valid);
    assert!(!initial_doc.warnings.is_empty());

    // Create workspace
    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: Some("python-api".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };
    let plan = install_service.create_plan(&request).unwrap();
    let _ = install_service.install(&plan).unwrap();

    // After init -> healthy
    let after_doc = workspace_service.doctor(&temp_dir.to_string_lossy()).unwrap();
    assert!(after_doc.is_valid);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_phase21_cli_cache_clean_flow() {
    let storage = InMemoryStorageRepository;
    let result = storage.clear_cache();
    assert!(result.is_ok());
}
