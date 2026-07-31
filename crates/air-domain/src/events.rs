use serde::{Deserialize, Serialize};

/// Real-time progress event emitted during installation / download workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressEvent {
    Initializing { target_dir: String },
    ResolvingDependencies { skill_count: usize },
    DownloadingSkill { skill_id: String, percentage: u8 },
    VerifyingChecksum { skill_id: String },
    MergingMarkdown { file_name: String },
    GeneratingWorkspace { path: String },
    Completed { summary: String },
    Failed { reason: String },
}
