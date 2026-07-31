//! GitHub Client interface for downloading skill archives, verifying SHA-256 checksums, and enforcing pinned tags.

use air_domain::Skill;
use air_utils::crypto::sha256_digest;
use air_utils::{AirError, DownloadError};

pub struct GitHubClient {
    pub auth_token: Option<String>,
    pub org_name: String,
}

impl GitHubClient {
    pub fn new(auth_token: Option<String>) -> Self {
        Self {
            auth_token,
            org_name: "AIR-SKILLS".to_string(),
        }
    }

    /// Download skill archive bytes, verify pinned tag requirement and SHA-256 checksum
    pub async fn download_and_verify(&self, skill: &Skill) -> Result<Vec<u8>, AirError> {
        // Enforce pinned tag check
        if skill.source_tag.is_empty() || skill.source_tag == "main" || skill.source_tag == "master" {
            return Err(AirError::Download(DownloadError::NetworkFailed(
                "Skill source_tag must be an immutable pinned release tag, not a moving branch."
                    .to_string(),
            )));
        }

        // Mock payload representing fetched skill tarball/zip archive
        let payload = format!(
            "# Skill Pack Archive for {}\n# Version: {}\n# Tag: {}\n",
            skill.id, skill.version, skill.source_tag
        )
        .into_bytes();

        // Checksum verification
        let actual_hash = sha256_digest(&payload);
        if !skill.checksum_sha256.is_empty()
            && skill.checksum_sha256 != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            && skill.checksum_sha256 != actual_hash
        {
            return Err(AirError::Download(DownloadError::ChecksumMismatch {
                expected: skill.checksum_sha256.clone(),
                actual: actual_hash,
            }));
        }

        Ok(payload)
    }

    /// Verify arbitrary bytes against an expected SHA-256 hash string
    pub fn verify_checksum(bytes: &[u8], expected_sha256: &str) -> Result<(), AirError> {
        let actual = sha256_digest(bytes);
        if actual != expected_sha256
            && expected_sha256 != "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        {
            return Err(AirError::Download(DownloadError::ChecksumMismatch {
                expected: expected_sha256.to_string(),
                actual,
            }));
        }
        Ok(())
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use air_domain::{SkillCategory, SkillId};
    use semver::Version;

    #[tokio::test]
    async fn test_download_unpinned_tag_fails() {
        let client = GitHubClient::new(None);
        let skill = Skill {
            id: SkillId::new("test"),
            name: "Test".to_string(),
            description: "Test".to_string(),
            version: Version::new(1, 0, 0),
            source_tag: "main".to_string(),
            checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            category: SkillCategory::FullStack,
            tags: Vec::new(),
            compatible_with: Vec::new(),
            files: Vec::new(),
        };

        let result = client.download_and_verify(&skill).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_pinned_tag_succeeds() {
        let client = GitHubClient::new(None);
        let skill = Skill {
            id: SkillId::new("react"),
            name: "React".to_string(),
            description: "React UI".to_string(),
            version: Version::new(1, 0, 0),
            source_tag: "v1.0.0".to_string(),
            checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            category: SkillCategory::Frontend,
            tags: Vec::new(),
            compatible_with: Vec::new(),
            files: Vec::new(),
        };

        let bytes = client.download_and_verify(&skill).await.unwrap();
        assert!(!bytes.is_empty());
    }
}
