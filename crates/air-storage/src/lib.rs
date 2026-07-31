//! Persistence abstraction for SQLite caching, installed skill metadata, and workspace history.

use air_domain::Skill;
use air_utils::AirError;

/// Storage repository trait interface
pub trait StorageRepository: Send + Sync {
    fn save_skill_cache(&self, skill: &Skill) -> Result<(), AirError>;
    fn get_cached_skill(&self, id: &str, version: &str) -> Result<Option<Skill>, AirError>;
    fn clear_cache(&self) -> Result<(), AirError>;
}

/// In-memory storage repository implementation stub for testing and foundation initialization
#[derive(Default, Clone)]
pub struct InMemoryStorageRepository;

impl StorageRepository for InMemoryStorageRepository {
    fn save_skill_cache(&self, _skill: &Skill) -> Result<(), AirError> {
        Ok(())
    }

    fn get_cached_skill(&self, _id: &str, _version: &str) -> Result<Option<Skill>, AirError> {
        Ok(None)
    }

    fn clear_cache(&self) -> Result<(), AirError> {
        Ok(())
    }
}
