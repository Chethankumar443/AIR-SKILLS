use air_merge::MergeEngine;

#[test]
fn test_golden_snapshot_single_section() {
    let engine = MergeEngine::new(false);
    let result = engine
        .merge_sections("System Architecture", &[("core", "Clean Architecture specification.")])
        .unwrap();

    assert!(result.contains("## System Architecture"));
    assert!(result.contains("Clean Architecture specification."));
}

#[test]
fn test_snapshot_multi_skill_deduplication() {
    let engine = MergeEngine::new(false);
    let skill_a = "## Setup\n- Run npm install\n- Set PORT=3000\n";
    let skill_b = "## Setup\n- Run npm install\n- Set DATABASE_URL=postgres://\n";

    let (merged, audit) = engine
        .merge_markdown_files("Project Environment Setup", &[("react", skill_a), ("postgres", skill_b)])
        .unwrap();

    assert!(merged.contains("# Project Environment Setup"));
    assert!(merged.contains("<!-- Source: react -->"));
    assert!(merged.contains("<!-- Source: postgres -->"));
    assert!(merged.contains("Set DATABASE_URL=postgres://"));
    assert_eq!(audit.total_sections_merged, 1);
}
