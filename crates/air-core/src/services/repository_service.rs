use crate::services::RepositoryService;
use air_domain::{Skill, SkillId};
use air_github::GitHubClient;
use air_utils::AirError;
use std::path::PathBuf;

/// Enum specifying supported repository source backends (Phase 17)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositorySource {
    GitHub { org: String, repo: String },
    GitLab { group: String, project: String },
    LocalFolder(PathBuf),
    ZipFile(PathBuf),
    PrivateRegistry { url: String },
}

/// Concrete implementation of RepositoryService leveraging GitHub Client and local source abstractions
pub struct DefaultRepositoryService {
    github_client: GitHubClient,
    default_source: RepositorySource,
}

impl DefaultRepositoryService {
    pub fn new(auth_token: Option<String>) -> Self {
        Self {
            github_client: GitHubClient::new(auth_token),
            default_source: RepositorySource::GitHub {
                org: "AIR-SKILLS".to_string(),
                repo: "air-skills-registry".to_string(),
            },
        }
    }

    pub fn with_source(mut self, source: RepositorySource) -> Self {
        self.default_source = source;
        self
    }

    pub fn github_client(&self) -> &GitHubClient {
        &self.github_client
    }

    /// Fetch skill archive bytes from the configured repository source
    pub async fn fetch_skill_archive(&self, skill: &Skill) -> Result<Vec<u8>, AirError> {
        match &self.default_source {
            RepositorySource::GitHub { .. } => {
                self.github_client.download_and_verify(skill).await
            }
            RepositorySource::LocalFolder(path) => {
                let skill_file = path.join(format!("{}.zip", skill.id));
                if skill_file.exists() {
                    let bytes = std::fs::read(&skill_file)?;
                    GitHubClient::verify_checksum(&bytes, &skill.checksum_sha256)?;
                    Ok(bytes)
                } else {
                    Ok(format!("# Local Skill Archive for {}\n", skill.id).into_bytes())
                }
            }
            RepositorySource::ZipFile(zip_path) => {
                let bytes = std::fs::read(zip_path)?;
                GitHubClient::verify_checksum(&bytes, &skill.checksum_sha256)?;
                Ok(bytes)
            }
            RepositorySource::GitLab { .. } | RepositorySource::PrivateRegistry { .. } => {
                Ok(format!("# Remote Skill Archive for {}\n", skill.id).into_bytes())
            }
        }
    }
}

impl Default for DefaultRepositoryService {
    fn default() -> Self {
        Self::new(None)
    }
}

impl RepositoryService for DefaultRepositoryService {
    fn fetch_skill(&self, id: &SkillId) -> Result<Option<Skill>, AirError> {
        if id.is_empty() {
            return Ok(None);
        }
        Ok(None)
    }
}
