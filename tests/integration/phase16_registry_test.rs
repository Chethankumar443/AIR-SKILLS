use air_core::DefaultRegistryService;
use air_domain::SkillId;
use std::fs;

#[test]
fn test_phase16_registry_search_and_lookup() {
    let registry = DefaultRegistryService::new();

    // 1. Search by keyword
    let python_skills = registry.search("python");
    assert!(!python_skills.is_empty());
    assert!(python_skills.iter().any(|s| s.id.as_str() == "python"));

    let ai_skills = registry.search("ai");
    assert!(ai_skills.len() >= 3);

    // 2. get_skill lookup
    let fastapi = registry.get_skill(&SkillId::new("fastapi")).unwrap();
    assert_eq!(fastapi.name, "FastAPI");
    assert!(registry.get_skill(&SkillId::new("unknown_skill")).is_err());

    // 3. get_starter_kit lookup
    let react_saas_kit = registry.get_starter_kit("react-saas").unwrap();
    assert_eq!(react_saas_kit.name, "Modern React SaaS");
    assert!(registry.get_starter_kit("unknown_kit").is_err());
}

#[test]
fn test_phase16_registry_format_validation() {
    let registry = DefaultRegistryService::new();

    let temp_base = std::env::temp_dir().join("air_phase16_val");
    let _ = fs::remove_dir_all(&temp_base);
    fs::create_dir_all(&temp_base).unwrap();

    // Build valid skill directory layout
    let skill_dir = temp_base.join("my-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    let valid_yaml = r#"{
        "id": "my-skill",
        "name": "My Skill",
        "version": "1.0.0",
        "source_tag": "v1.0.0",
        "checksum_sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    }"#;

    fs::write(skill_dir.join("skill.yaml"), valid_yaml).unwrap();
    fs::write(skill_dir.join("README.md"), "# Skill Docs").unwrap();
    fs::write(skill_dir.join("system.md"), "# AI System Prompt").unwrap();
    fs::write(skill_dir.join("design.md"), "# AI Design Rules").unwrap();
    fs::create_dir_all(skill_dir.join("templates")).unwrap();
    fs::create_dir_all(skill_dir.join("hooks")).unwrap();

    let report = registry.validate(&skill_dir).unwrap();
    assert!(report.is_valid, "Expected valid skill pack, errors: {:?}", report.errors);

    let _ = fs::remove_dir_all(&temp_base);
}
