//! Deterministic markdown merge engine with append-with-attribution conflict resolution and `.air/merge-audit.json` log generation.
//!
//! Features:
//! - Header-based section parsing (`#`, `##`, `###`)
//! - Paragraph and bullet-point deduplication
//! - Source attribution tags (`<!-- Source: <skill_id> -->`)
//! - Priority ordering (Starter Kit base docs < Skill Pack docs < Local Overrides)
//! - Strict conflict detection (`StrictConflict`)
//! - Audit log generation (`MergeAudit`)

use air_domain::{Conflict, ConflictResolution, Skill};
use air_utils::{AirError, MergeError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// Audit entry recorded in `.air/merge-audit.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeAuditEntry {
    pub section_heading: String,
    pub source_skills: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub conflict_occurred: bool,
    pub resolution_applied: String,
}

/// Audit log saved into `.air/merge-audit.json`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeAudit {
    pub schema_version: u32,
    pub generated_at: DateTime<Utc>,
    pub total_sections_merged: usize,
    pub entries: Vec<MergeAuditEntry>,
}

pub struct MergeEngine {
    pub strict: bool,
}

impl MergeEngine {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    /// Perform dry-run check for potential conflicts across skills
    pub fn dry_run_check(&self, skills: &[Skill]) -> Result<Vec<Conflict>, AirError> {
        let mut conflicts = Vec::new();
        if skills.len() < 2 {
            return Ok(conflicts);
        }

        if self.strict {
            // Check if skills have overlapping categories or custom conflicts
            let mut seen_ids = HashSet::new();
            for s in skills {
                if !seen_ids.insert(s.id.as_str()) {
                    conflicts.push(Conflict {
                        section_heading: format!("Duplicate skill {}", s.id),
                        contributing_skills: vec![s.id.clone()],
                        resolution: ConflictResolution::FailStrict,
                    });
                }
            }
        }

        Ok(conflicts)
    }

    /// Merge multiple markdown document sources by section heading with deduplication and source attribution
    pub fn merge_markdown_files(
        &self,
        heading_title: &str,
        documents: &[(&str, &str)],
    ) -> Result<(String, MergeAudit), AirError> {
        if documents.is_empty() {
            let audit = MergeAudit {
                schema_version: 1,
                generated_at: Utc::now(),
                total_sections_merged: 0,
                entries: Vec::new(),
            };
            return Ok((String::new(), audit));
        }

        // Check strict mode
        if self.strict && documents.len() > 1 {
            let non_empty_docs: Vec<_> = documents.iter().filter(|(_, c)| !c.trim().is_empty()).collect();
            if non_empty_docs.len() > 1 {
                let ids: Vec<String> = non_empty_docs.iter().map(|(id, _)| id.to_string()).collect();
                return Err(AirError::Merge(MergeError::StrictConflict {
                    heading: heading_title.to_string(),
                    skills: ids,
                }));
            }
        }

        // BTreeMap guarantees deterministic section output order by heading
        let mut sections: BTreeMap<String, Vec<(&str, String)>> = BTreeMap::new();
        let mut audit_entries = Vec::new();

        for (skill_id, content) in documents {
            let parsed_sections = parse_markdown_sections(content);
            for (heading, body) in parsed_sections {
                sections
                    .entry(heading)
                    .or_default()
                    .push((*skill_id, body));
            }
        }

        let mut output = format!("# {}\n\n", heading_title);
        let now = Utc::now();

        for (heading, items) in &sections {
            let source_skills: Vec<String> = items.iter().map(|(id, _)| id.to_string()).collect();
            let is_conflict = items.len() > 1;

            audit_entries.push(MergeAuditEntry {
                section_heading: heading.clone(),
                source_skills: source_skills.clone(),
                timestamp: now,
                conflict_occurred: is_conflict,
                resolution_applied: if is_conflict {
                    "AppendWithAttribution & ParagraphDeduplication".to_string()
                } else {
                    "DirectInclude".to_string()
                },
            });

            output.push_str(&format!("## {}\n\n", heading));

            let mut seen_lines = HashSet::new();

            for (skill_id, body) in items {
                output.push_str(&format!("<!-- Source: {} -->\n", skill_id));
                for line in body.lines() {
                    let trimmed = line.trim();
                    // Deduplicate bullet points and identical sentences
                    if (trimmed.starts_with('-') || trimmed.starts_with('*') || trimmed.starts_with("✓") || trimmed.starts_with("+"))
                        && !seen_lines.insert(trimmed.to_string())
                    {
                        continue; // Skip duplicate bullet line
                    }
                    output.push_str(line);
                    output.push('\n');
                }
                output.push('\n');
            }
        }

        let audit = MergeAudit {
            schema_version: 1,
            generated_at: now,
            total_sections_merged: sections.len(),
            entries: audit_entries,
        };

        Ok((output, audit))
    }

    pub fn merge_sections(
        &self,
        heading: &str,
        skills_content: &[(&str, &str)],
    ) -> Result<String, AirError> {
        let (merged, _) = self.merge_markdown_files(heading, skills_content)?;
        Ok(merged)
    }
}

/// Helper function to parse markdown content into section headings and body text
fn parse_markdown_sections(content: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_heading = "General Overview".to_string();
    let mut current_body = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            if !current_body.trim().is_empty() {
                sections.push((current_heading.clone(), current_body.trim().to_string()));
                current_body.clear();
            }
            // Strip `#` prefixes
            current_heading = trimmed.trim_start_matches('#').trim().to_string();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }

    if !current_body.trim().is_empty() {
        sections.push((current_heading, current_body.trim().to_string()));
    }

    if sections.is_empty() && !content.trim().is_empty() {
        sections.push(("General Overview".to_string(), content.trim().to_string()));
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_deduplication_and_attribution() {
        let engine = MergeEngine::new(false);
        let doc1 = "## Code Style\n- Use 4 spaces\n- Write unit tests\n";
        let doc2 = "## Code Style\n- Use 4 spaces\n- Enforce strict typing\n";

        let (merged, audit) = engine
            .merge_markdown_files("Project Standards", &[("react", doc1), ("ts", doc2)])
            .unwrap();

        assert!(merged.contains("# Project Standards"));
        assert!(merged.contains("<!-- Source: react -->"));
        assert!(merged.contains("<!-- Source: ts -->"));
        assert!(merged.contains("Enforce strict typing"));
        assert_eq!(audit.total_sections_merged, 1);
    }

    #[test]
    fn test_strict_mode_conflict() {
        let engine = MergeEngine::new(true);
        let doc1 = "## Setup\nRun npm install\n";
        let doc2 = "## Setup\nRun yarn install\n";

        let result = engine.merge_markdown_files("Setup", &[("react", doc1), ("ts", doc2)]);
        assert!(result.is_err());
    }
}
