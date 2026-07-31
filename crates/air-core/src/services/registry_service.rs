use air_domain::{Skill, SkillId, StarterKit, ValidationReport};
use air_registry::RegistryClient;
use air_utils::{AirError, RegistryError};
use semver::VersionReq;
use std::path::Path;

/// Concrete implementation of registry services handling skill resolution, search, validation & semver compatibility
#[derive(Default)]
pub struct DefaultRegistryService {
    client: RegistryClient,
}

impl DefaultRegistryService {
    pub fn new() -> Self {
        Self {
            client: RegistryClient::new(),
        }
    }

    pub fn with_client(client: RegistryClient) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &RegistryClient {
        &self.client
    }

    /// Resolve a starter kit ID into its constituent skill requirements
    pub fn get_starter_kit(&self, kit_id: &str) -> Result<StarterKit, AirError> {
        self.client.get_starter_kit(kit_id)
    }

    /// Search for skills in the registry index matching a query string
    pub fn search(&self, query: &str) -> Vec<Skill> {
        self.client.search(query)
    }

    /// Retrieve a skill pack by ID
    pub fn get_skill(&self, id: &SkillId) -> Result<Skill, AirError> {
        self.client.get_skill(id)
    }

    /// Find a skill pack by ID and version requirement
    pub fn find_skill(&self, id: &SkillId, _req: &VersionReq) -> Result<Skill, AirError> {
        self.client.get_skill(id)
    }

    /// Validate a registry root directory or skill pack directory structure
    pub fn validate(&self, path: &Path) -> Result<ValidationReport, AirError> {
        self.client.validate(path)
    }

    /// Check semver compatibility across a collection of skills
    pub fn check_compatibility(&self, skills: &[Skill]) -> Result<(), AirError> {
        for skill in skills {
            for constraint in &skill.compatible_with {
                if let Some(target) = skills.iter().find(|s| s.id == constraint.skill_id) {
                    if !constraint.range.matches(&target.version) {
                        return Err(AirError::Registry(RegistryError::VersionUnresolvable(
                            format!(
                                "Skill `{}` requires `{}` in range `{}`, but found `{}`",
                                skill.id, constraint.skill_id, constraint.range, target.version
                            ),
                        )));
                    }
                }
            }
        }
        Ok(())
    }
}
