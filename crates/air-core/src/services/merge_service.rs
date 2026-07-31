use crate::services::MergeService;
use air_domain::{Skill, ValidationResult};
use air_merge::MergeEngine;
use air_utils::AirError;

/// Concrete implementation of MergeService using air-merge engine
pub struct DefaultMergeService {
    engine: MergeEngine,
}

impl DefaultMergeService {
    pub fn new(strict: bool) -> Self {
        Self {
            engine: MergeEngine::new(strict),
        }
    }
}

impl Default for DefaultMergeService {
    fn default() -> Self {
        Self::new(false)
    }
}

impl MergeService for DefaultMergeService {
    fn merge(&self, skills: &[Skill]) -> Result<ValidationResult, AirError> {
        let conflicts = self.engine.dry_run_check(skills)?;
        if !conflicts.is_empty() && self.engine.strict {
            let conflict_msgs: Vec<String> = conflicts
                .into_iter()
                .map(|c| format!("Conflict in heading: {}", c.section_heading))
                .collect();
            return Ok(ValidationResult::with_errors(conflict_msgs));
        }
        Ok(ValidationResult::success())
    }
}
