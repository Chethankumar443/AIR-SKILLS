use air_tui::{ScreenState, TuiApp};

#[test]
fn test_tui_app_banner_and_screen_states() {
    let mut app = TuiApp::new(true);

    let banner = app.render_banner();
    assert!(banner.contains("AIR.SKILLS"));

    // Check Home screen state initialization
    if let ScreenState::Home { ref target_dir } = app.current_screen {
        assert_eq!(target_dir, ".");
    } else {
        panic!("Expected Home screen state");
    }
}
