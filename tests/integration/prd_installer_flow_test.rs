use air_tui::{TuiApp, WizardStep};

#[test]
fn test_keyboard_wizard_step_initialization() {
    // Plain-text mode (no ANSI). Initial step is Splash since new startup flow was added.
    let app = TuiApp::new(true);

    assert_eq!(app.current_step, WizardStep::Splash);

    let banner = app.render_banner();
    // Plain-mode banner contains both required strings
    assert!(banner.contains("AIR.SKILLS v1.0.0"));
    assert!(banner.contains("AI Project Bootstrap & Skill Manager"));
}
