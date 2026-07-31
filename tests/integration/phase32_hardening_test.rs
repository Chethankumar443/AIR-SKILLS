use air_core::{
    DefaultInstallService, DefaultRegistryService, DefaultUninstallService, DefaultWorkspaceService,
    InstallRequest, InstallService, UninstallService, WorkspaceService,
};
use air_github::GitHubClient;
use air_registry::RegistryClient;
use air_domain::SkillId;
use std::fs;
use std::path::Path;

#[test]
fn test_safe_uninstall_preserves_user_code() {
    let temp_dir = std::env::temp_dir().join("air_test_uninstall_preserve");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Create fake user project file
    let user_file = temp_dir.join("src_main.rs");
    fs::write(&user_file, "fn main() { println!(\"Hello World\"); }").unwrap();

    // Initialize AIR workspace
    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: Some("python-api".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };
    let plan = install_service.create_plan(&request).unwrap();
    let _ = install_service.install(&plan).unwrap();

    assert!(temp_dir.join(".air/lock.json").exists());

    // Perform safe uninstall (without purging generated)
    let uninstall_service = DefaultUninstallService::new();
    let result = uninstall_service.uninstall(&temp_dir.to_string_lossy(), false).unwrap();

    assert!(result.success);
    assert!(!temp_dir.join(".air").exists());
    // User file MUST exist untouched
    assert!(user_file.exists());
    assert_eq!(fs::read_to_string(&user_file).unwrap(), "fn main() { println!(\"Hello World\"); }");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_safe_uninstall_purge_generated() {
    let temp_dir = std::env::temp_dir().join("air_test_uninstall_purge");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: Some("react-saas".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };
    let plan = install_service.create_plan(&request).unwrap();
    let _ = install_service.install(&plan).unwrap();

    assert!(temp_dir.join("system_instructions.md").exists());
    assert!(temp_dir.join("design.md").exists());

    let uninstall_service = DefaultUninstallService::new();
    let result = uninstall_service.uninstall(&temp_dir.to_string_lossy(), true).unwrap();

    assert!(result.success);
    assert!(!temp_dir.join(".air").exists());
    assert!(!temp_dir.join("system_instructions.md").exists());
    assert!(!temp_dir.join("design.md").exists());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_checksum_mismatch_rejection() {
    let client = GitHubClient::new();
    let temp_dir = std::env::temp_dir().join("air_test_checksum_mismatch");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Invalid SHA-256 digest
    let invalid_sha256 = "0000000000000000000000000000000000000000000000000000000000000000";
    let res = client.download_and_verify("Chethankumar443/AIR-SKILLS", "v1.0.0", invalid_sha256, &temp_dir);

    assert!(res.is_err());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_invalid_skill_yaml_validation() {
    let client = RegistryClient::new();
    let temp_dir = std::env::temp_dir().join("air_test_invalid_skill_yaml");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Missing checksum_sha256 & source_tag
    let invalid_yaml = r#"
id: invalid-skill
name: Invalid Skill
version: 1.0.0
"#;
    fs::write(temp_dir.join("skill.yaml"), invalid_yaml).unwrap();

    let report = client.validate(&temp_dir);
    assert!(!report.is_valid);
    assert!(report.errors.iter().any(|e| e.contains("checksum_sha256") || e.contains("source_tag")));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_reinstall_safe_workspace() {
    let temp_dir = std::env::temp_dir().join("air_test_reinstall_safe");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let install_service = DefaultInstallService::new();

    // Initial Install
    let req1 = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: None,
        custom_skills: vec![SkillId::new("react")],
        strict_merge: false,
    };
    let plan1 = install_service.create_plan(&req1).unwrap();
    let _ = install_service.install(&plan1).unwrap();

    // Re-install with extra skill
    let req2 = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: None,
        custom_skills: vec![SkillId::new("react"), SkillId::new("typescript")],
        strict_merge: false,
    };
    let plan2 = install_service.create_plan(&req2).unwrap();
    let res2 = install_service.install(&plan2).unwrap();

    assert!(res2.success);
    let lock_content = fs::read_to_string(temp_dir.join(".air/lock.json")).unwrap();
    assert!(lock_content.contains("react"));
    assert!(lock_content.contains("typescript"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_broken_lock_file_doctor_detection() {
    let temp_dir = std::env::temp_dir().join("air_test_broken_lock_doc");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    fs::create_dir_all(temp_dir.join(".air")).unwrap();
    fs::write(temp_dir.join(".air/lock.json"), "{ invalid_json }").unwrap();

    let workspace_service = DefaultWorkspaceService::new();
    let report = workspace_service.doctor(&temp_dir.to_string_lossy()).unwrap();

    assert!(!report.is_valid);
    assert!(report.errors.iter().any(|e| e.contains("lock.json")));

    let _ = fs::remove_dir_all(&temp_dir);
}
