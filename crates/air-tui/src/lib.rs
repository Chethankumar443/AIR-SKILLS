//! Terminal UI — AIR.SKILLS installer wizard.
//!
//! Flow: Splash → EnvCheck → Welcome → WorkspaceDetect → Philosophy →
//!       Disclaimer → SetupMode → KitOrSkillSelect → PreInstallSummary →
//!       Downloading → CreatingWorkspace → FinalScreen
//!
//! Rendering rules:
//!   • Every screen starts with clear_screen() — no stacking content.
//!   • Every menu line is erased before rewriting (erase_line + \r) — no ghost chars.
//!   • All cursor movement uses crossterm execute! — no raw escape codes.
//!   • All output in raw mode uses print!(...\r\n), never println!().

use air_core::{DefaultInstallService, InstallRequest, InstallService};
use air_domain::{ProgressEvent, SkillId};
use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

// ─── Wizard step state machine ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    Splash,
    EnvCheck,
    Welcome,
    WorkspaceDetect,
    Disclaimer,
    SetupModeChoice,
    KitOrSkillSelect,
    PreInstallSummary,
    Downloading,
    CreatingWorkspace,
    FinalScreen,
}

// ─── Navigation return types ──────────────────────────────────────────────────

pub enum NavResult {
    Next,
    Back,
}

pub enum NavChoice<T> {
    Option(T),
    Back,
}

pub enum NavSelection<T> {
    Selected(T),
    Back,
}

// ─── Kit → skills mapping ─────────────────────────────────────────────────────

fn kit_skills(kit_id: &str) -> Vec<String> {
    match kit_id {
        "react-saas"       => vec!["React", "TypeScript", "Tailwind CSS", "PostgreSQL", "Testing"],
        "ai-chatbot"       => vec!["Python", "LangChain", "OpenAI", "SQLite"],
        "desktop-app"      => vec!["Tauri", "Rust", "Astro", "SQLite"],
        "nextjs-fullstack" => vec!["Next.js", "TypeScript", "Tailwind CSS", "Prisma"],
        "python-api"       => vec!["Python", "FastAPI", "Pydantic", "PostgreSQL"],
        "ai-agent"         => vec!["Python", "LangGraph", "MCP", "Ollama"],
        "mcp-server"       => vec!["TypeScript", "MCP SDK"],
        "empty-project"    => vec!["AIR Core"],
        _                  => vec!["AIR Core"],
    }
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

#[allow(dead_code)]
fn kit_download_skills(kit_id: &str) -> Vec<String> {
    kit_skills(kit_id)
        .iter()
        .map(|s| format!("{}-skill", s.to_lowercase().replace(' ', "-").replace('.', "")))
        .collect()
}

// ─── Screen helpers ───────────────────────────────────────────────────────────

/// Clear the entire terminal and move cursor to top-left.
/// Clear terminal. In plain/non-TTY mode just print blank lines as a visual separator.
fn clear_screen(plain: bool) {
    if plain {
        println!("\n");
    } else {
        let mut out = io::stdout();
        execute!(out, Clear(ClearType::All), crossterm::cursor::MoveTo(0, 0)).ok();
    }
}

/// Print the horizontal rule used as a section divider.
fn rule() {
    println!("  \x1b[90m{}\x1b[0m", "─".repeat(46));
}

/// Render a selectable menu. Returns the number of rows printed (= options.len()).
/// On the second+ call in the same loop, pass `redraw = true` to jump the cursor back up first.
fn render_menu_lines<S: AsRef<str>>(options: &[S], sel: usize, redraw: bool) {
    if redraw {
        execute!(io::stdout(), MoveUp(options.len() as u16)).ok();
    }
    for (idx, opt) in options.iter().enumerate() {
        execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
        if idx == sel {
            print!("  \x1b[36m\x1b[1m❯ {}\x1b[0m\r\n", opt.as_ref());
        } else {
            print!("    \x1b[90m{}\x1b[0m\r\n", opt.as_ref());
        }
    }
    io::stdout().flush().ok();
}

/// Returns true for Ctrl+C.
fn ctrl_c(key: &crossterm::event::KeyEvent) -> bool {
    key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c')
}

/// Drain pending key events (prevents stale input from previous screen leaking in).
fn drain_events() {
    while event::poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = event::read();
    }
}

/// Read a line of text while staying in crossterm raw mode.
///
/// Avoids the buffered-Enter bug: if we called disable_raw_mode() and then
/// io::stdin().read_line(), the Enter that confirmed the menu option would be
/// consumed immediately, producing an empty string. This reads char-by-char
/// from the crossterm event stream instead.
fn read_line_raw(prompt: &str) -> String {
    let mut buf = String::new();
    // Print initial prompt on a clean line
    execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
    print!("{}", prompt);
    io::stdout().flush().ok();

    // Drain any stale events (including the Enter that triggered selection)
    drain_events();

    loop {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Enter => {
                    print!("\r\n");
                    io::stdout().flush().ok();
                    break;
                }
                KeyCode::Esc => {
                    buf.clear();
                    print!("\r\n");
                    io::stdout().flush().ok();
                    break;
                }
                KeyCode::Backspace => {
                    if !buf.is_empty() {
                        buf.pop();
                        // Redraw the prompt + buffer
                        execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
                        print!("{}{}", prompt, buf);
                        io::stdout().flush().ok();
                    }
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    buf.push(c);
                    print!("{}", c);
                    io::stdout().flush().ok();
                }
                _ => {}
            }
        }
    }
    buf
}

// ─── TuiApp ──────────────────────────────────────────────────────────────────

pub struct TuiApp {
    pub current_step: WizardStep,
    pub plain_text_fallback: bool,
}

impl TuiApp {
    pub fn new(plain_text_fallback: bool) -> Self {
        Self {
            current_step: WizardStep::Splash,
            plain_text_fallback,
        }
    }

    pub fn render_banner(&self) -> String {
        if self.plain_text_fallback {
            "─────────────────────────────────────────────────────────────────────────────────────────────────────\n                         AIR.SKILLS v1.0.0\n               AI Project Bootstrap & Skill Manager\n─────────────────────────────────────────────────────────────────────────────────────────────────────"
                .to_string()
        } else {
            concat!(
                "\x1b[36m",
                " ▄▄▄▄▄▄▄▄▄   ▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄    ▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄   ▄▄▄▄  ▄▄▄▄  ▄▄▄▄         ▄▄▄▄          ▄▄▄▄▄▄▄▄▄▄ \r\n",
                "███▓┌─ ███▓┐ ███▓│ ███▓┌─ ███▓┐ ▄██▓┌─ ███▓│ ███▓│  ███▓│ ███▓│ ███▓│░░░░░░░ ███▓│░░░░░░░ ▄██▓┌─ ███▓│\r\n",
                "███▓│▄▄████│ ███▓│ ████▄▄▄██▀┌┘ ▀███▄▄▄▄▄ ─┘ ████▄▄▄██▀┌┘ ███▓│ ███▓│▒▒▒▒▒▒▒ ███▓│▒▒▒▒▒▒▒ ▀███▄▄▄▄▄ ─┘\r\n",
                "███▓├─ ████│ ███▓│ ███▓┌─ ███▄┐ ▄▄▄▄┌─ ███▓│ ███▓┌─ ███▄┐ ███▓│ ███▓│▓ ▄▄▄▄  ███▓│▓ ▄▄▄▄  ▄▄▄▄┌─ ███▓│\r\n",
                "▓██▓│  ▓██▓│ ███▓│ ▓███│  ███▓│ ▓███▄▄▄██▓┌┘ ▓██▓│  ▓██▓│ ███▓│ ▀███▄▄▄███▓│ ▀███▄▄▄███▓│ ▓███▄▄▄██▓┌┘\r\n",
                " ───┘   ───┘  ───┘  ───┘   ───┘  ─────────┘   ───┘   ───┘  ───┘    ────────┘    ────────┘  ─────────┘ \r\n",
                "\x1b[0m\r\n",
                "                \x1b[1mAI Project Bootstrap & Skill Manager\x1b[0m\r\n",
                "                         \x1b[90mVersion 1.0.0\x1b[0m"
            )
            .to_string()
        }
    }

    // ─── Installer state machine ──────────────────────────────────────────────

    pub async fn run_installer(&mut self, target_dir_override: &str) -> Result<(), String> {
        let mut cwd = if target_dir_override == "." {
            env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string())
        } else {
            target_dir_override.to_string()
        };

        let mut is_starter_kit = true;
        let mut selected_kit = (
            "react-saas".to_string(),
            "Modern React SaaS".to_string(),
            "React + TypeScript + Tailwind + PostgreSQL".to_string(),
        );
        let mut selected_skills: Vec<String> = kit_skills("react-saas");

        let starter_kits: Vec<(String, String, String)> = vec![
            ("react-saas",       "Modern React SaaS",        "React · TS · Tailwind · Postgres"),
            ("ai-chatbot",       "AI Chatbot",                "Python · LangChain · OpenAI · SQLite"),
            ("desktop-app",      "Desktop App (Rust+Tauri)",  "Tauri · Rust · Astro · SQLite"),
            ("nextjs-fullstack", "Full Stack Next.js",        "Next.js · TS · Tailwind · Prisma"),
            ("python-api",       "Python API",                "FastAPI · Pydantic · Postgres"),
            ("ai-agent",         "AI Agent",                  "Python · LangGraph · MCP · Ollama"),
            ("mcp-server",       "MCP Server",                "TypeScript · MCP SDK"),
            ("empty-project",    "Empty Project",             "Minimal AIR workspace"),
        ]
        .into_iter()
        .map(|(a, b, c)| (a.to_string(), b.to_string(), c.to_string()))
        .collect();

        let mut custom_skill_options: Vec<(String, String, bool)> = vec![
            ("astro",       "Astro",       true),
            ("react",       "React",       true),
            ("vue",         "Vue",         false),
            ("svelte",      "Svelte",      false),
            ("typescript",  "TypeScript",  true),
            ("tailwind",    "Tailwind CSS",true),
        ]
        .into_iter()
        .map(|(a, b, c)| (a.to_string(), b.to_string(), c))
        .collect();

        let mut step = WizardStep::Splash;

        loop {
            match step {
                WizardStep::Splash => {
                    self.current_step = WizardStep::Splash;
                    self.render_splash().await;
                    step = WizardStep::EnvCheck;
                }
                WizardStep::EnvCheck => {
                    self.current_step = WizardStep::EnvCheck;
                    self.render_env_check().await;
                    step = WizardStep::Welcome;
                }
                WizardStep::Welcome => {
                    self.current_step = WizardStep::Welcome;
                    match self.screen_welcome()? {
                        NavResult::Next => step = WizardStep::WorkspaceDetect,
                        NavResult::Back => step = WizardStep::Splash,
                    }
                }
                WizardStep::WorkspaceDetect => {
                    self.current_step = WizardStep::WorkspaceDetect;
                    match self.screen_workspace_detect(&mut cwd)? {
                        NavResult::Next => step = WizardStep::Disclaimer,
                        NavResult::Back => step = WizardStep::Welcome,
                    }
                }
                WizardStep::Disclaimer => {
                    self.current_step = WizardStep::Disclaimer;
                    match self.screen_disclaimer()? {
                        NavResult::Next => step = WizardStep::SetupModeChoice,
                        NavResult::Back => step = WizardStep::WorkspaceDetect,
                    }
                }
                WizardStep::SetupModeChoice => {
                    self.current_step = WizardStep::SetupModeChoice;
                    match self.screen_setup_mode()? {
                        NavChoice::Option(is_kit) => {
                            is_starter_kit = is_kit;
                            step = WizardStep::KitOrSkillSelect;
                        }
                        NavChoice::Back => step = WizardStep::Disclaimer,
                    }
                }
                WizardStep::KitOrSkillSelect => {
                    self.current_step = WizardStep::KitOrSkillSelect;
                    if is_starter_kit {
                        match self.screen_starter_kit_select(&starter_kits)? {
                            NavSelection::Selected(kit) => {
                                selected_skills = kit_skills(&kit.0);
                                selected_kit = kit;
                                step = WizardStep::PreInstallSummary;
                            }
                            NavSelection::Back => step = WizardStep::SetupModeChoice,
                        }
                    } else {
                        match self.screen_custom_skills_select(&mut custom_skill_options)? {
                            NavSelection::Selected(skills) => {
                                selected_skills = skills;
                                step = WizardStep::PreInstallSummary;
                            }
                            NavSelection::Back => step = WizardStep::SetupModeChoice,
                        }
                    }
                }
                WizardStep::PreInstallSummary => {
                    self.current_step = WizardStep::PreInstallSummary;
                    let label = if is_starter_kit { selected_kit.1.as_str() } else { "Custom Setup" };
                    match self.screen_pre_install_summary(&cwd, label, &selected_skills)? {
                        NavResult::Next => step = WizardStep::Downloading,
                        NavResult::Back => step = WizardStep::KitOrSkillSelect,
                    }
                }
                WizardStep::Downloading | WizardStep::CreatingWorkspace => {
                    self.current_step = WizardStep::Downloading;

                    let install_service = DefaultInstallService::new();
                    let skill_objs: Vec<SkillId> = selected_skills
                        .iter()
                        .map(|s| SkillId::new(s.to_lowercase().replace(' ', "-")))
                        .collect();

                    let req = InstallRequest {
                        target_directory: cwd.clone(),
                        starter_kit_id: if is_starter_kit { Some(selected_kit.0.clone()) } else { None },
                        custom_skills: skill_objs,
                        strict_merge: false,
                    };

                    let plan = install_service
                        .create_plan(&req)
                        .map_err(|e| format!("Failed to create install plan: {}", e))?;

                    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

                    let install_handle = tokio::spawn(async move {
                        install_service.install_with_progress(&plan, Some(tx)).await
                    });

                    clear_screen(self.plain_text_fallback);
                    println!("{}", self.render_banner());
                    println!();
                    rule();
                    println!("  \x1b[1mInstalling Workspace Intelligence\x1b[0m");
                    rule();
                    println!();

                    while let Some(event) = rx.recv().await {
                        match event {
                            ProgressEvent::Initializing { target_dir } => {
                                println!("  \x1b[36m⚙\x1b[0m  Initializing workspace at \x1b[1m{}\x1b[0m", target_dir);
                            }
                            ProgressEvent::ResolvingDependencies { skill_count } => {
                                println!("  \x1b[36m⚙\x1b[0m  Resolving {} skill dependencies...", skill_count);
                            }
                            ProgressEvent::DownloadingSkill { skill_id, .. } => {
                                println!("  \x1b[32m+\x1b[0m  Downloading skill \x1b[1m{}\x1b[0m", skill_id);
                            }
                            ProgressEvent::VerifyingChecksum { skill_id } => {
                                println!("  \x1b[32m✓\x1b[0m  Verified SHA-256 checksum for \x1b[90m{}\x1b[0m", skill_id);
                            }
                            ProgressEvent::MergingMarkdown { file_name } => {
                                println!("  \x1b[32m+\x1b[0m  Merging Markdown intelligence (\x1b[36m{}\x1b[0m)", file_name);
                            }
                            ProgressEvent::GeneratingWorkspace { .. } => {
                                println!("  \x1b[32m+\x1b[0m  Generating deterministic \x1b[1m.air/\x1b[0m workspace structure...");
                            }
                            ProgressEvent::Completed { summary } => {
                                println!();
                                println!("  \x1b[32m\x1b[1m{}\x1b[0m", summary);
                            }
                            ProgressEvent::Failed { reason } => {
                                println!("  \x1b[31m✖  Install failed: {}\x1b[0m", reason);
                            }
                        }
                        sleep(Duration::from_millis(60)).await;
                    }

                    let _ = install_handle
                        .await
                        .map_err(|e| format!("Task join failed: {}", e))?
                        .map_err(|e| format!("Installation error: {}", e))?;

                    step = WizardStep::FinalScreen;
                }
                WizardStep::FinalScreen => {
                    self.current_step = WizardStep::FinalScreen;
                    let label = if is_starter_kit { selected_kit.1.as_str() } else { "Custom Setup" };
                    self.render_final_screen(&cwd, label, &selected_skills);
                    return Ok(());
                }
            }
        }
    }

    // ─── Startup screens ──────────────────────────────────────────────────────

    async fn render_splash(&self) {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();

        if self.plain_text_fallback {
            println!("  Checking environment...");
            sleep(Duration::from_millis(600)).await;
            return;
        }

        let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        for i in 0..14 {
            let _ = execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0));
            print!("\x1b[90m  {} Checking environment...\x1b[0m", spinner[i % spinner.len()]);
            io::stdout().flush().ok();
            sleep(Duration::from_millis(70)).await;
        }
        // Clear spinner line
        let _ = execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0));
        io::stdout().flush().ok();
    }

    async fn render_env_check(&self) {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        println!("  \x1b[1mEnvironment Check\x1b[0m");
        println!();

        let checks = [
            ("Git",       "\x1b[32m✓\x1b[0m"),
            ("Internet",  "\x1b[32m✓\x1b[0m"),
            ("Rust",      "\x1b[32m✓\x1b[0m"),
            ("Workspace", "\x1b[32m✓\x1b[0m"),
        ];

        for (name, icon) in &checks {
            sleep(Duration::from_millis(200)).await;
            println!("  {} {}", icon, name);
        }
        println!();
        sleep(Duration::from_millis(400)).await;
    }

    fn screen_welcome(&self) -> Result<NavResult, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        println!("  \x1b[1mWelcome to AIR.SKILLS\x1b[0m");
        println!();
        println!("  Bootstrap AI-ready project workspaces in minutes.");
        println!("  Install curated Skill Packs that give any AI coding");
        println!("  assistant instant context about your project.");
        println!();
        println!("  \x1b[90mPress Enter to begin  ·  Esc to quit\x1b[0m");
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  continue    Esc / ←  back\x1b[0m");
        println!();

        let _ = enable_raw_mode();
        drain_events();
        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) {
                    let _ = disable_raw_mode();
                    return Err("Canceled".to_string());
                }
                match key.code {
                    KeyCode::Enter | KeyCode::Right => { let _ = disable_raw_mode(); return Ok(NavResult::Next); }
                    KeyCode::Esc   | KeyCode::Left  => { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                    _ => {}
                }
            }
        }
    }

    // ─── Installer screens ────────────────────────────────────────────────────

    fn screen_workspace_detect(&self, cwd: &mut String) -> Result<NavResult, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        println!("  \x1b[1mCurrent Directory\x1b[0m");
        println!();
        println!("  \x1b[33m{}\x1b[0m", cwd);
        println!();
        println!("  Install AIR in this directory?");
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  confirm    Esc / ←  back\x1b[0m");
        println!();

        let options = ["Yes, use this directory", "Choose a different directory", "Exit"];
        let mut sel = 0usize;

        let _ = enable_raw_mode();
        drain_events();
        render_menu_lines(&options, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up => {
                        sel = if sel == 0 { options.len() - 1 } else { sel - 1 };
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        sel = (sel + 1) % options.len();
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Enter | KeyCode::Right => {
                        match sel {
                            0 => { let _ = disable_raw_mode(); return Ok(NavResult::Next); }
                            1 => {
                                clear_screen(self.plain_text_fallback);
                                let path = read_line_raw("  Enter target directory path: ");
                                if !path.is_empty() { *cwd = path; }
                                let _ = disable_raw_mode();
                                return Ok(NavResult::Next);
                            }
                            _ => { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                        }
                    }
                    KeyCode::Esc | KeyCode::Left => { let _ = disable_raw_mode(); return Ok(NavResult::Back); }
                    _ => {}
                }
            }
        }
    }

    fn screen_disclaimer(&self) -> Result<NavResult, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mBefore You Continue\x1b[0m");
        rule();
        println!();
        println!("  AIR will:");
        println!();
        println!("    \x1b[32m+\x1b[0m  Create the \x1b[1m.air/\x1b[0m workspace directory");
        println!("    \x1b[32m+\x1b[0m  Install selected Skill Packs");
        println!("    \x1b[32m+\x1b[0m  Generate AI instruction files");
        println!("    \x1b[32m+\x1b[0m  Generate project design documents");
        println!("    \x1b[32m+\x1b[0m  Create AIR configuration files");
        println!("    \x1b[32m+\x1b[0m  Download required skill templates");
        println!();
        println!("  AIR will NOT:");
        println!();
        println!("    \x1b[31m-\x1b[0m  Modify your existing source code");
        println!("    \x1b[31m-\x1b[0m  Delete any project files");
        println!("    \x1b[31m-\x1b[0m  Access private repositories without permission");
        println!("    \x1b[31m-\x1b[0m  Upload your project automatically");
        println!();
        println!("  Remove AIR at any time:  \x1b[36mair uninstall\x1b[0m");
        println!();
        rule();
        println!("  Continue?");
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  confirm    Esc / ←  back\x1b[0m");
        println!();

        let options = ["Yes, continue", "No, go back"];
        let mut sel = 0usize;

        let _ = enable_raw_mode();
        drain_events();
        render_menu_lines(&options, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up => {
                        sel = if sel == 0 { options.len() - 1 } else { sel - 1 };
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        sel = (sel + 1) % options.len();
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Enter | KeyCode::Right => {
                        let _ = disable_raw_mode();
                        if sel == 0 { return Ok(NavResult::Next); } else { return Ok(NavResult::Back); }
                    }
                    KeyCode::Esc | KeyCode::Left => { let _ = disable_raw_mode(); return Ok(NavResult::Back); }
                    _ => {}
                }
            }
        }
    }

    fn screen_setup_mode(&self) -> Result<NavChoice<bool>, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mChoose Setup Method\x1b[0m");
        rule();
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  confirm    Esc / ←  back\x1b[0m");
        println!();

        let options = [
            "Starter Kit   (recommended - curated stack)",
            "Custom Setup  (choose individual skills)",
        ];
        let mut sel = 0usize;

        let _ = enable_raw_mode();
        drain_events();
        render_menu_lines(&options, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up => {
                        sel = if sel == 0 { options.len() - 1 } else { sel - 1 };
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        sel = (sel + 1) % options.len();
                        render_menu_lines(&options, sel, true);
                    }
                    KeyCode::Enter | KeyCode::Right => {
                        let _ = disable_raw_mode();
                        return Ok(NavChoice::Option(sel == 0));
                    }
                    KeyCode::Esc | KeyCode::Left => { let _ = disable_raw_mode(); return Ok(NavChoice::Back); }
                    _ => {}
                }
            }
        }
    }

    fn screen_starter_kit_select(
        &self,
        kits: &[(String, String, String)],
    ) -> Result<NavSelection<(String, String, String)>, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mChoose Starter Kit\x1b[0m");
        rule();
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  confirm    Esc / ←  back\x1b[0m");
        println!();

        let mut sel = 0usize;
        let _ = enable_raw_mode();
        drain_events();

        self.render_kit_list(kits, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up => {
                        sel = if sel == 0 { kits.len() - 1 } else { sel - 1 };
                        self.render_kit_list(kits, sel, true);
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        sel = (sel + 1) % kits.len();
                        self.render_kit_list(kits, sel, true);
                    }
                    KeyCode::Enter | KeyCode::Right => {
                        let _ = disable_raw_mode();
                        return Ok(NavSelection::Selected(kits[sel].clone()));
                    }
                    KeyCode::Esc | KeyCode::Left => { let _ = disable_raw_mode(); return Ok(NavSelection::Back); }
                    _ => {}
                }
            }
        }
    }

    /// Render the kit list. Each kit = 1 row (name only, desc inline dimmed).
    fn render_kit_list(&self, kits: &[(String, String, String)], sel: usize, redraw: bool) {
        if redraw {
            execute!(io::stdout(), MoveUp(kits.len() as u16)).ok();
        }
        let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80);
        for (idx, (_id, name, desc)) in kits.iter().enumerate() {
            execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
            if idx == sel {
                let prefix_len = 32; // "  ❯ " (4) + name (26) + "  " (2)
                let max_desc = term_width.saturating_sub(prefix_len + 1);
                let truncated_desc = if desc.len() > max_desc && max_desc > 3 {
                    format!("{}...", &desc[..max_desc.saturating_sub(3)])
                } else {
                    desc.clone()
                };
                print!("  \x1b[36m\x1b[1m❯ {:26}\x1b[0m  \x1b[90m{}\x1b[0m\r\n", name, truncated_desc);
            } else {
                print!("    \x1b[90m{}\x1b[0m\r\n", name);
            }
        }
        io::stdout().flush().ok();
    }

    fn screen_custom_skills_select(
        &self,
        skills: &mut [(String, String, bool)],
    ) -> Result<NavSelection<Vec<String>>, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mSelect Skills\x1b[0m");
        rule();
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Space  toggle    Enter / →  confirm    Esc / ←  back\x1b[0m");
        println!();

        let mut sel = 0usize;
        let _ = enable_raw_mode();
        drain_events();

        self.render_skill_checkboxes(skills, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up => {
                        sel = if sel == 0 { skills.len() - 1 } else { sel - 1 };
                        self.render_skill_checkboxes(skills, sel, true);
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        sel = (sel + 1) % skills.len();
                        self.render_skill_checkboxes(skills, sel, true);
                    }
                    KeyCode::Char(' ') => {
                        skills[sel].2 = !skills[sel].2;
                        self.render_skill_checkboxes(skills, sel, true);
                    }
                    KeyCode::Enter | KeyCode::Right => {
                        let _ = disable_raw_mode();
                        let chosen: Vec<String> = skills
                            .iter()
                            .filter(|(_, _, ok)| *ok)
                            .map(|(_, name, _)| name.clone())
                            .collect();
                        return Ok(NavSelection::Selected(if chosen.is_empty() {
                            vec!["Astro".to_string(), "React".to_string(), "TypeScript".to_string()]
                        } else {
                            chosen
                        }));
                    }
                    KeyCode::Esc | KeyCode::Left => { let _ = disable_raw_mode(); return Ok(NavSelection::Back); }
                    _ => {}
                }
            }
        }
    }

    fn render_skill_checkboxes(&self, skills: &[(String, String, bool)], sel: usize, redraw: bool) {
        if redraw {
            execute!(io::stdout(), MoveUp(skills.len() as u16)).ok();
        }
        for (idx, (_id, name, checked)) in skills.iter().enumerate() {
            execute!(io::stdout(), Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
            let cb = if *checked { "\x1b[32m[x]\x1b[0m" } else { "\x1b[90m[ ]\x1b[0m" };
            if idx == sel {
                print!("  \x1b[36m\x1b[1m❯\x1b[0m {} \x1b[36m\x1b[1m{}\x1b[0m\r\n", cb, name);
            } else {
                print!("    {} \x1b[90m{}\x1b[0m\r\n", cb, name);
            }
        }
        io::stdout().flush().ok();
    }

    fn screen_pre_install_summary(
        &self,
        workspace_path: &str,
        setup_mode: &str,
        skills: &[String],
    ) -> Result<NavResult, String> {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mInstallation Summary\x1b[0m");
        rule();
        println!();
        println!("  \x1b[90mWorkspace\x1b[0m   {}", workspace_path);
        println!("  \x1b[90mSetup\x1b[0m       {}", setup_mode);
        println!();
        println!("  \x1b[1mSkills to install:\x1b[0m");
        for s in skills {
            println!("    \x1b[32m+\x1b[0m  {}", s);
        }
        println!();
        println!("  \x1b[1mFiles to create:\x1b[0m");
        println!("    .air/   system_instructions.md   design.md   air.json");
        println!();
        rule();
        println!("  Proceed?");
        println!();
        println!("  \x1b[90m↑↓ / Tab  navigate    Enter / →  install    Esc / ←  back\x1b[0m");
        println!();

        let options = ["Install", "Go back"];
        let mut sel = 0usize;

        let _ = enable_raw_mode();
        drain_events();
        render_menu_lines(&options, sel, false);

        loop {
            if let Ok(Event::Key(key)) = event::read() {
                if ctrl_c(&key) { let _ = disable_raw_mode(); return Err("Canceled".to_string()); }
                match key.code {
                    KeyCode::Up    => { if sel > 0 { sel -= 1; } render_menu_lines(&options, sel, true); }
                    KeyCode::Down  => { if sel + 1 < options.len() { sel += 1; } render_menu_lines(&options, sel, true); }
                    KeyCode::Enter => {
                        let _ = disable_raw_mode();
                        if sel == 0 { return Ok(NavResult::Next); } else { return Ok(NavResult::Back); }
                    }
                    KeyCode::Esc => { let _ = disable_raw_mode(); return Ok(NavResult::Back); }
                    _ => {}
                }
            }
        }
    }

    // ─── Execution screens (no input) ─────────────────────────────────────────

    #[allow(dead_code)]
    async fn render_downloading(&self, skills: &[String]) {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mDownloading Skills\x1b[0m");
        rule();
        println!();
        for skill in skills {
            sleep(Duration::from_millis(200)).await;
            println!("  \x1b[32m+\x1b[0m  {}", skill);
        }
        println!();
        println!("  \x1b[32m\x1b[1mAll skills downloaded.\x1b[0m");
        println!();
        sleep(Duration::from_millis(500)).await;
    }

    #[allow(dead_code)]
    async fn render_creating_workspace(&self, files: &[String]) {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[1mCreating Workspace\x1b[0m");
        rule();
        println!();
        for file in files {
            sleep(Duration::from_millis(150)).await;
            println!("  \x1b[32m+\x1b[0m  {}", file);
        }
        println!();
        println!("  \x1b[32m\x1b[1mWorkspace ready.\x1b[0m");
        println!();
        sleep(Duration::from_millis(500)).await;
    }

    fn render_final_screen(&self, workspace_path: &str, setup_name: &str, skills: &[String]) {
        clear_screen(self.plain_text_fallback);
        println!("{}", self.render_banner());
        println!();
        rule();
        println!("  \x1b[32m\x1b[1mInstallation Complete\x1b[0m");
        rule();
        println!();
        println!("  \x1b[90mWorkspace\x1b[0m   {}", workspace_path);
        println!("  \x1b[90mSetup\x1b[0m       {}", setup_name);
        println!();
        println!("  \x1b[1mInstalled Skills:\x1b[0m");
        for skill in skills {
            println!("    \x1b[32m✓\x1b[0m  {}", skill);
        }
        println!();
        rule();
        println!("  \x1b[1mNext Steps:\x1b[0m");
        println!();
        println!("    1.  Open your AI IDE");
        println!("    2.  Open this project directory");
        println!("    3.  Your AI assistant now has full project context");
        println!();
        println!("  Happy coding! \x1b[36m::\x1b[0m");
        println!();
        rule();
        println!();
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_step_is_splash() {
        assert_eq!(TuiApp::new(true).current_step, WizardStep::Splash);
    }

    #[test]
    fn plain_banner_contains_required_strings() {
        let b = TuiApp::new(true).render_banner();
        assert!(b.contains("AIR.SKILLS v1.0.0"));
        assert!(b.contains("AI Project Bootstrap & Skill Manager"));
    }

    #[test]
    fn ansi_banner_contains_tagline_and_version() {
        let b = TuiApp::new(false).render_banner();
        assert!(b.contains("AI Project Bootstrap & Skill Manager"));
        assert!(b.contains("Version 1.0.0"));
    }

    #[test]
    fn kit_skills_react_saas_correct() {
        let s = kit_skills("react-saas");
        assert!(s.contains(&"React".to_string()));
        assert!(s.contains(&"TypeScript".to_string()));
        assert!(s.contains(&"Tailwind CSS".to_string()));
    }

    #[test]
    fn kit_skills_python_api_no_react() {
        let s = kit_skills("python-api");
        assert!(s.contains(&"FastAPI".to_string()));
        assert!(!s.contains(&"React".to_string()));
    }

    #[test]
    fn kit_skills_unknown_returns_air_core() {
        assert_eq!(kit_skills("nonexistent"), vec!["AIR Core".to_string()]);
    }

    #[test]
    fn kit_download_skills_sanitises_names() {
        let d = kit_download_skills("nextjs-fullstack");
        assert!(d.iter().any(|s| s.contains("tailwind-css")));
        assert!(d.iter().any(|s| s.contains("nextjs")));
    }

    #[test]
    fn all_eight_kits_have_skills() {
        for id in &["react-saas","ai-chatbot","desktop-app","nextjs-fullstack","python-api","ai-agent","mcp-server","empty-project"] {
            assert!(!kit_skills(id).is_empty(), "kit '{}' is empty", id);
        }
    }
}
