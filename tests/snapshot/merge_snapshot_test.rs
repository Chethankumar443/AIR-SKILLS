use air_merge::MergeEngine;

#[test]
fn test_golden_snapshot_single_section() {
    let engine = MergeEngine::new(false);
    let result = engine
        .merge_sections("System Architecture", &[("core", "Clean Architecture specification.")])
        .unwrap();

    let expected = "## System Architecture\n\nClean Architecture specification.\n";
    assert_eq!(result, expected);
}
