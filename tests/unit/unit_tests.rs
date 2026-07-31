use air_domain::{Skill, SkillCategory, SkillId};
use air_utils::sha256_digest;
use semver::Version;

#[test]
fn test_domain_model_creation() {
    let skill = Skill {
        id: SkillId::new("astro"),
        version: Version::new(1, 0, 0),
        source_tag: "v1.0.0".to_string(),
        checksum_sha256: "dummy_checksum".to_string(),
        category: SkillCategory::Frontend,
        compatible_with: Vec::new(),
        files: Vec::new(),
    };

    assert_eq!(skill.id.as_str(), "astro");
    assert_eq!(skill.version.to_string(), "1.0.0");
}

#[test]
fn test_sha256_digest() {
    let digest = sha256_digest(b"AIR.SKILLS");
    assert!(!digest.is_empty());
}
