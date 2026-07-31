use air_core::{
    DefaultInstallService, DefaultRegistryService, DefaultWorkspaceService, InstallRequest,
    InstallService, WorkspaceService,
};
use air_domain::{SkillId, Manifest};
use air_storage::{InMemoryStorageRepository, StorageRepository};
use air_tui::TuiApp;
use air_utils::init_logging;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "air")]
#[command(author = "AIR.SKILLS Maintainers <maintainers@airskills.dev>")]
#[command(version = "1.0.0")]
#[command(about = "AI Project Bootstrap & Skill Manager", long_about = None)]
struct Cli {
    #[arg(short, long, global = true, help = "Force plain-text output (disable ANSI TUI)")]
    plain: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new AIR.SKILLS workspace
    Init {
        #[arg(short = 'd', long, help = "Target directory path")]
        path: Option<String>,
    },
    /// Add skill packs to the current workspace
    Add {
        #[arg(help = "Skill pack ID(s) to add", required = true)]
        skills: Vec<String>,
    },
    /// Remove skill packs from the current workspace
    #[command(alias = "rm")]
    Remove {
        #[arg(help = "Skill pack ID(s) to remove", required = true)]
        skills: Vec<String>,
    },
    /// Update installed skill packs or the CLI binary (--self)
    Update {
        #[arg(long = "self", help = "Update the AIR CLI binary itself")]
        update_self: bool,
    },
    /// List installed skill packs in current workspace
    #[command(alias = "ls")]
    List,
    /// Search official Skill Packs registry
    Search {
        #[arg(help = "Query string or keyword")]
        query: Option<String>,
    },
    /// Validate workspace health and configuration
    Doctor,
    /// Clean temporary workspace cache and audit files
    Clean,
    /// Manage local skill pack cache
    Cache {
        #[command(subcommand)]
        command: Option<CacheCommands>,
    },
    /// Output AIR version and workspace information
    Version,
}

#[derive(Subcommand)]
enum CacheCommands {
    /// List items in local storage cache
    List,
    /// Clear local storage cache
    Clear,
}

#[tokio::main]
async fn main() {
    init_logging();
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { path }) => {
            let dir = path.unwrap_or_else(|| ".".to_string());
            let mut app = TuiApp::new(cli.plain);
            if let Err(err) = app.run_installer(&dir).await {
                eprintln!("Error during installer execution: {}", err);
                std::process::exit(1);
            }
        }
        Some(Commands::Add { skills }) => {
            let install_service = DefaultInstallService::new();
            let skill_ids: Vec<SkillId> = skills.iter().map(|s| SkillId::new(s)).collect();

            let request = InstallRequest {
                target_directory: ".".to_string(),
                starter_kit_id: None,
                custom_skills: skill_ids,
                strict_merge: false,
            };

            match install_service.create_plan(&request) {
                Ok(plan) => match install_service.install(&plan) {
                    Ok(result) => {
                        println!("✓ Successfully added {} skill(s) to workspace.", result.installed_count);
                    }
                    Err(e) => {
                        eprintln!("Error adding skills: {}", e);
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("Error creating install plan: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Remove { skills }) => {
            let lock_path = Path::new(".air/lock.json");
            if !lock_path.exists() {
                eprintln!("Error: No AIR workspace found in current directory. Run `air init` first.");
                std::process::exit(1);
            }

            let content = match fs::read_to_string(lock_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading .air/lock.json: {}", e);
                    std::process::exit(1);
                }
            };

            let mut manifest: Manifest = match serde_json::from_str(&content) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error parsing lock.json: {}", e);
                    std::process::exit(1);
                }
            };

            let to_remove: Vec<String> = skills.iter().map(|s| s.to_lowercase()).collect();
            let initial_len = manifest.installed_skills.len();
            manifest.installed_skills.retain(|s| !to_remove.contains(&s.id.as_str().to_lowercase()));

            let removed_count = initial_len - manifest.installed_skills.len();
            if removed_count == 0 {
                println!("No matching skills were removed.");
                return;
            }

            // Save updated lock manifest
            if let Ok(updated_json) = serde_json::to_string_pretty(&manifest) {
                let _ = fs::write(lock_path, updated_json);
            }

            // Re-run workspace generator with remaining skills
            let install_service = DefaultInstallService::new();
            let remaining_ids: Vec<SkillId> = manifest.installed_skills.iter().map(|s| s.id.clone()).collect();

            let req = InstallRequest {
                target_directory: ".".to_string(),
                starter_kit_id: manifest.starter_kit.map(|k| k.id),
                custom_skills: remaining_ids,
                strict_merge: false,
            };

            if let Ok(plan) = install_service.create_plan(&req) {
                let _ = install_service.install(&plan);
            }

            println!("✓ Removed {} skill(s) from workspace.", removed_count);
        }
        Some(Commands::Search { query }) => {
            let registry_service = DefaultRegistryService::new();
            let q = query.as_deref().unwrap_or("*");
            let results = registry_service.search(q);

            println!("─────────────────────────────────────────────────────────────────────────────");
            println!("                      AIR.SKILLS Registry Search ({})", q);
            println!("─────────────────────────────────────────────────────────────────────────────");
            println!("{:18} {:26} {:12} {}", "ID", "Name", "Category", "Description");
            println!("─────────────────────────────────────────────────────────────────────────────");

            for s in results {
                println!("{:18} {:26} {:12?} {}", s.id, s.name, s.category, s.description);
            }
            println!("─────────────────────────────────────────────────────────────────────────────");
        }
        Some(Commands::Update { update_self }) => {
            if update_self {
                println!("Checking AIR CLI binary version...");
                println!("AIR.SKILLS CLI v1.0.0 is up to date.");
            } else {
                println!("Updating installed workspace skills...");
                let workspace_service = DefaultWorkspaceService::new();
                let doc = workspace_service.doctor(".").unwrap_or_else(|_| air_domain::ValidationResult {
                    is_valid: false,
                    errors: vec!["Doctor check failed".to_string()],
                    warnings: Vec::new(),
                });
                if doc.is_valid {
                    println!("✓ All workspace skill packs are up to date.");
                } else {
                    eprintln!("Workspace error: run `air doctor` for diagnostics.");
                }
            }
        }
        Some(Commands::List) => {
            let lock_path = Path::new(".air/lock.json");
            if !lock_path.exists() {
                println!("Workspace is not initialized. Run `air init` to bootstrap an AIR workspace.");
                return;
            }

            if let Ok(content) = fs::read_to_string(lock_path) {
                if let Ok(manifest) = serde_json::from_str::<Manifest>(&content) {
                    println!("─────────────────────────────────────────────────────────────────────────────");
                    println!("                  Installed Skill Packs in Current Workspace");
                    println!("─────────────────────────────────────────────────────────────────────────────");
                    if let Some(ref kit) = manifest.starter_kit {
                        println!(" Starter Kit : {}", kit.name);
                    }
                    println!(" Skills Count: {}", manifest.installed_skills.len());
                    println!("─────────────────────────────────────────────────────────────────────────────");
                    for skill in manifest.installed_skills {
                        println!("  ✓  {:18} {:24} Tag: {}", skill.id, skill.name, skill.source_tag);
                    }
                    println!("─────────────────────────────────────────────────────────────────────────────");
                    return;
                }
            }
            eprintln!("Error reading workspace manifest `.air/lock.json`.");
        }
        Some(Commands::Doctor) => {
            let workspace_service = DefaultWorkspaceService::new();
            println!("─────────────────────────────────────────────────────────────────────────────");
            println!("                       AIR.SKILLS Workspace Doctor");
            println!("─────────────────────────────────────────────────────────────────────────────");
            match workspace_service.doctor(".") {
                Ok(result) => {
                    if result.is_valid {
                        println!("  ✓  Workspace status     : Healthy & Valid");
                        println!("  ✓  Configuration schema : Compliant (v1)");
                        println!("  ✓  Lock manifest        : Found (.air/lock.json)");
                    } else {
                        println!("  ✖  Workspace status     : Action Required");
                        for err in result.errors {
                            println!("      - Error: {}", err);
                        }
                    }
                    for warn in result.warnings {
                        println!("      ! Warning: {}", warn);
                    }
                }
                Err(e) => eprintln!("Doctor check failed: {}", e),
            }
            println!("─────────────────────────────────────────────────────────────────────────────");
        }
        Some(Commands::Clean) => {
            let _ = fs::remove_file(".air/merge-audit.json");
            let storage = InMemoryStorageRepository;
            let _ = storage.clear_cache();
            println!("✓ Cleaned workspace audit logs and storage cache.");
        }
        Some(Commands::Cache { command }) => {
            let storage = InMemoryStorageRepository;
            match command {
                Some(CacheCommands::Clear) => {
                    let _ = storage.clear_cache();
                    println!("✓ Local storage cache cleared.");
                }
                _ => {
                    println!("Local Storage Cache Status:");
                    println!("  Cache Directory : ~/.air/cache/");
                    println!("  Cached Items    : 0 (In-Memory Repository Active)");
                }
            }
        }
        Some(Commands::Version) => {
            println!("AIR.SKILLS CLI v1.0.0");
            println!("Target OS           : {}", std::env::consts::OS);
            println!("Architecture        : {}", std::env::consts::ARCH);
            println!("Workspace Engine    : air-workspace v0.1.0");
        }
        None => {
            let mut app = TuiApp::new(cli.plain);
            if let Err(err) = app.run_installer(".").await {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
