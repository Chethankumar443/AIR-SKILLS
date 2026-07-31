use air_core::{
    DefaultInstallService, DefaultRegistryService, DefaultWorkspaceService, InstallRequest,
    InstallService, WorkspaceService,
};

#[test]
fn test_phase15_registry_starter_kit_lookup() {
    let registry = DefaultRegistryService::new();
    let kit = registry.get_starter_kit("react-webapp").unwrap();

    assert_eq!(kit.id, "react-webapp");
    assert_eq!(kit.skills.len(), 2);
}

#[test]
fn test_phase15_install_plan_creation() {
    let install_service = DefaultInstallService::new();
    let request = InstallRequest {
        target_directory: "target/test_workspace".to_string(),
        starter_kit_id: Some("react-webapp".to_string()),
        custom_skills: Vec::new(),
        strict_merge: false,
    };

    let plan = install_service.create_plan(&request).unwrap();
    assert_eq!(plan.starter_kit_id, Some("react-webapp".to_string()));
    assert_eq!(plan.resolved_skills.len(), 2);
}

#[test]
fn test_phase15_workspace_doctor_diagnostic() {
    let workspace_service = DefaultWorkspaceService::new();
    let result = workspace_service.doctor(".").unwrap();

    // Doctor should return valid for existing workspace directory
    assert!(result.is_valid);
}
