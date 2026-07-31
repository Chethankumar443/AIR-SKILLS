use air_merge::MergeEngine;

#[test]
fn test_merge_engine_attribution() {
    let engine = MergeEngine::new(false);
    let section1 = ("skill-a", "Rule 1: Always test code.");
    let section2 = ("skill-b", "Rule 2: Keep architecture clean.");

    let merged = engine
        .merge_sections("Coding Rules", &[section1, section2])
        .unwrap();

    assert!(merged.contains("### Coding Rules (from: skill-a)"));
    assert!(merged.contains("### Coding Rules (from: skill-b)"));
}
