// SPDX-License-Identifier: MPL-2.0
//! # Did You Actually Do That?
//!
//! A verification framework for validating claimed AI actions against actual outcomes.
//!
//! Born from the frustration of AI systems claiming to perform actions without
//! actually doing them. This library provides types and verification logic to
//! hold systems (and ourselves) accountable.
//!
//! ## Core Concepts
//!
//! - **Claim**: An assertion that an action was performed
//! - **Evidence**: Observable artifacts that should exist if the claim is true
//! - **Verification**: The process of checking evidence against claims
//! - **Verdict**: The outcome of verification (Confirmed, Refuted, Inconclusive)
//!
//! ## Features
//!
//! - `async` - Enable async verification for network-based evidence checks (HTTP, TCP)

#[cfg(feature = "async")]
pub mod async_checks;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during verification
#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Evidence not found: {0}")]
    EvidenceNotFound(String),

    #[error("Evidence mismatch: expected {expected}, found {found}")]
    EvidenceMismatch { expected: String, found: String },

    #[error("Verification timed out after {0} seconds")]
    Timeout(u64),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid claim structure: {0}")]
    InvalidClaim(String),
}

/// The verdict of a verification check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    /// All evidence confirms the claim
    Confirmed,
    /// Evidence contradicts the claim
    Refuted,
    /// Insufficient evidence to determine truth
    Inconclusive,
    /// Verification could not be performed
    Unverifiable,
}

impl Verdict {
    pub fn is_trustworthy(&self) -> bool {
        matches!(self, Verdict::Confirmed)
    }
}

/// Types of evidence that can be checked
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum EvidenceSpec {
    /// A file should exist at the given path
    FileExists { path: String },

    /// A file should exist with specific content hash
    FileWithHash { path: String, sha256: String },

    /// A file should contain the given substring
    FileContains { path: String, substring: String },

    /// A directory should exist
    DirectoryExists { path: String },

    /// A command should succeed (exit code 0)
    CommandSucceeds { command: String, args: Vec<String> },

    /// Custom predicate (for extensibility)
    Custom {
        name: String,
        params: HashMap<String, String>,
    },
}

/// A claim that some action was performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Unique identifier for this claim (auto-generated if not provided)
    #[serde(default = "Claim::default_id")]
    pub id: String,

    /// Human-readable description of what was claimed
    pub description: String,

    /// When the claim was made (defaults to now if not provided)
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,

    /// Evidence that should exist if the claim is true
    pub evidence: Vec<EvidenceSpec>,

    /// Optional context about who/what made the claim
    pub source: Option<String>,
}

impl Claim {
    pub fn new(description: impl Into<String>) -> Self {
        let description_string = description.into();
        let claim_id = Self::generate_id(&description_string);
        Self {
            id: claim_id,
            description: description_string,
            timestamp: Utc::now(),
            evidence: Vec::new(),
            source: None,
        }
    }

    fn default_id() -> String {
        Self::generate_id("unnamed-claim")
    }

    pub fn with_evidence(mut self, evidence: EvidenceSpec) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    fn generate_id(description: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(description.as_bytes());
        hasher.update(Utc::now().timestamp().to_le_bytes());
        hex::encode(&hasher.finalize()[..8])
    }
}

/// Result of verifying a single piece of evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceResult {
    pub spec: EvidenceSpec,
    pub verdict: Verdict,
    pub details: Option<String>,
}

/// Complete verification report for a claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub claim: Claim,
    pub evidence_results: Vec<EvidenceResult>,
    pub overall_verdict: Verdict,
    pub verified_at: DateTime<Utc>,
}

impl VerificationReport {
    /// Summary suitable for display
    pub fn summary(&self) -> String {
        let emoji = match self.overall_verdict {
            Verdict::Confirmed => "✓",
            Verdict::Refuted => "✗",
            Verdict::Inconclusive => "?",
            Verdict::Unverifiable => "⊘",
        };
        format!(
            "[{}] {} - {:?}",
            emoji, self.claim.description, self.overall_verdict
        )
    }
}

/// The main verifier that checks claims against reality
pub struct Verifier {
    /// Custom evidence checkers for extensibility
    custom_checkers:
        HashMap<String, Box<dyn Fn(&HashMap<String, String>) -> Result<Verdict, VerificationError>>>,
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            custom_checkers: HashMap::new(),
        }
    }

    /// Register a custom evidence checker
    pub fn register_checker<F>(&mut self, name: impl Into<String>, checker: F)
    where
        F: Fn(&HashMap<String, String>) -> Result<Verdict, VerificationError> + 'static,
    {
        self.custom_checkers.insert(name.into(), Box::new(checker));
    }

    /// Verify a single piece of evidence
    pub fn check_evidence(&self, evidence: &EvidenceSpec) -> EvidenceResult {
        let (verdict, details) = match evidence {
            EvidenceSpec::FileExists { path } => {
                if Path::new(path).exists() {
                    (Verdict::Confirmed, Some(format!("File exists: {}", path)))
                } else {
                    (Verdict::Refuted, Some(format!("File not found: {}", path)))
                }
            }

            EvidenceSpec::FileWithHash { path, sha256 } => match std::fs::read(path) {
                Ok(contents) => {
                    let mut hasher = Sha256::new();
                    hasher.update(&contents);
                    let actual_hash = hex::encode(hasher.finalize());
                    if actual_hash == *sha256 {
                        (Verdict::Confirmed, Some("Hash matches".to_string()))
                    } else {
                        (
                            Verdict::Refuted,
                            Some(format!(
                                "Hash mismatch: expected {}, got {}",
                                sha256, actual_hash
                            )),
                        )
                    }
                }
                Err(e) => (Verdict::Refuted, Some(format!("Cannot read file: {}", e))),
            },

            EvidenceSpec::FileContains { path, substring } => {
                match std::fs::read_to_string(path) {
                    Ok(contents) => {
                        if contents.contains(substring) {
                            (Verdict::Confirmed, Some("Substring found".to_string()))
                        } else {
                            (Verdict::Refuted, Some("Substring not found".to_string()))
                        }
                    }
                    Err(e) => (Verdict::Refuted, Some(format!("Cannot read file: {}", e))),
                }
            }

            EvidenceSpec::DirectoryExists { path } => {
                let p = Path::new(path);
                if p.exists() && p.is_dir() {
                    (
                        Verdict::Confirmed,
                        Some(format!("Directory exists: {}", path)),
                    )
                } else {
                    (
                        Verdict::Refuted,
                        Some(format!("Directory not found: {}", path)),
                    )
                }
            }

            EvidenceSpec::CommandSucceeds { command, args } => {
                match std::process::Command::new(command).args(args).output() {
                    Ok(output) => {
                        if output.status.success() {
                            (Verdict::Confirmed, Some("Command succeeded".to_string()))
                        } else {
                            (
                                Verdict::Refuted,
                                Some(format!(
                                    "Command failed with exit code: {:?}",
                                    output.status.code()
                                )),
                            )
                        }
                    }
                    Err(e) => (Verdict::Refuted, Some(format!("Command error: {}", e))),
                }
            }

            EvidenceSpec::Custom { name, params } => {
                if let Some(checker) = self.custom_checkers.get(name) {
                    match checker(params) {
                        Ok(v) => (v, None),
                        Err(e) => (Verdict::Unverifiable, Some(e.to_string())),
                    }
                } else {
                    (
                        Verdict::Unverifiable,
                        Some(format!("No checker for: {}", name)),
                    )
                }
            }
        };

        EvidenceResult {
            spec: evidence.clone(),
            verdict,
            details,
        }
    }

    /// Verify a complete claim
    pub fn verify(&self, claim: &Claim) -> VerificationReport {
        if claim.evidence.is_empty() {
            return VerificationReport {
                claim: claim.clone(),
                evidence_results: vec![],
                overall_verdict: Verdict::Unverifiable,
                verified_at: Utc::now(),
            };
        }

        let evidence_results: Vec<EvidenceResult> = claim
            .evidence
            .iter()
            .map(|e| self.check_evidence(e))
            .collect();

        // Overall verdict: all must confirm for Confirmed, any refuted = Refuted
        let overall_verdict = if evidence_results
            .iter()
            .all(|r| r.verdict == Verdict::Confirmed)
        {
            Verdict::Confirmed
        } else if evidence_results
            .iter()
            .any(|r| r.verdict == Verdict::Refuted)
        {
            Verdict::Refuted
        } else if evidence_results
            .iter()
            .all(|r| r.verdict == Verdict::Unverifiable)
        {
            Verdict::Unverifiable
        } else {
            Verdict::Inconclusive
        };

        VerificationReport {
            claim: claim.clone(),
            evidence_results,
            overall_verdict,
            verified_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_creation() {
        let claim = Claim::new("Created a file")
            .with_evidence(EvidenceSpec::FileExists {
                path: "/tmp/test.txt".to_string(),
            })
            .with_source("test-agent");

        assert!(!claim.id.is_empty());
        assert_eq!(claim.evidence.len(), 1);
        assert_eq!(claim.source, Some("test-agent".to_string()));
    }

    #[test]
    fn test_verdict_trustworthiness() {
        assert!(Verdict::Confirmed.is_trustworthy());
        assert!(!Verdict::Refuted.is_trustworthy());
        assert!(!Verdict::Inconclusive.is_trustworthy());
        assert!(!Verdict::Unverifiable.is_trustworthy());
    }
}
