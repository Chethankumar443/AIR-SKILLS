use air_core::{DefaultInstallService, InstallRequest, InstallService};
use air_domain::SkillId;
use std::fs;
use std::path::Path;

#[test]
fn test_golden_workspace_output_snapshot() {
    let temp_dir = std::env::temp_dir().join("air_golden_test");
    let _ = fs::remove_dir_all(&temp_dir);

    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: temp_dir.to_string_lossy().to_string(),
        starter_kit_id: None,
        custom_skills: vec![SkillId::new("react"), SkillId::new("typescript")],
        strict_merge: false,
    };

    let plan = install_service.create_plan(&request).unwrap();
    let result = install_service.install(&plan).unwrap();
    assert!(result.success);

    // Read generated files
    let system_instructions = fs::read_to_string(temp_dir.join("system_instructions.md")).unwrap();
    let design_docs = fs::read_to_string(temp_dir.join("design.md")).unwrap();
    let lock_json = fs::read_to_string(temp_dir.join(".air/lock.json")).unwrap();

    // Verify golden properties
    assert!(system_instructions.contains("# AIR System Instructions"));
    assert!(system_instructions.contains("<!-- Source: react -->"));
    assert!(system_instructions.contains("<!-- Source: typescript -->"));

    assert!(design_docs.contains("# AIR Architecture & Design Guidelines"));
    assert!(design_docs.contains("Component Architecture"));

    assert!(lock_json.contains("\"schema_version\": 1"));
    assert!(lock_json.contains("\"id\": \"react\""));
    assert!(lock_json.contains("\"id\": \"typescript\""));

    let _ = fs::remove_dir_all(&temp_dir);
}
