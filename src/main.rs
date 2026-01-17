// SPDX-License-Identifier: MPL-2.0
//! CLI for Did You Actually Do That?
//!
//! Usage:
//!   dyadt check <claim.json>     - Verify a claim from a JSON file
//!   dyadt verify <path>          - Quick check if a file/directory exists
//!   dyadt report <claims.json>   - Generate a verification report

use did_you_actually_do_that::{Claim, EvidenceSpec, VerificationReport, Verifier, Verdict};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::process::ExitCode;

fn print_help() {
    eprintln!(
        r#"
Did You Actually Do That? (dyadt) v0.1.0
A verification framework for validating claimed actions against reality.

USAGE:
    dyadt <COMMAND> [ARGS]

COMMANDS:
    check <claim.json>    Verify a claim from a JSON file
    verify <path>         Quick check if a file or directory exists
    hash <file>           Compute SHA-256 hash of a file (for evidence specs)
    report <claims.json>  Verify multiple claims and generate a report
    help                  Show this help message

EXAMPLES:
    # Verify a specific claim
    dyadt check my-claim.json

    # Quick existence check
    dyadt verify /path/to/expected/file.txt

    # Get hash for evidence specification
    dyadt hash important-file.rs

CLAIM JSON FORMAT:
    {{
        "description": "Created the configuration file",
        "evidence": [
            {{ "type": "FileExists", "spec": {{ "path": "/etc/myapp/config.toml" }} }},
            {{ "type": "FileContains", "spec": {{ "path": "/etc/myapp/config.toml", "substring": "version = " }} }}
        ],
        "source": "setup-agent"
    }}

EXIT CODES:
    0 - All claims verified (Confirmed)
    1 - One or more claims refuted
    2 - Inconclusive or unverifiable
    3 - Error (invalid input, etc.)
"#
    );
}

fn verify_claim_file(path: &str) -> ExitCode {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            return ExitCode::from(3);
        }
    };

    let claim: Claim = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing claim JSON: {}", e);
            return ExitCode::from(3);
        }
    };

    let verifier = Verifier::new();
    let report = verifier.verify(&claim);
    print_report(&report);

    verdict_to_exit_code(report.overall_verdict)
}

fn quick_verify(path: &str) -> ExitCode {
    let claim = Claim::new(format!("Path exists: {}", path))
        .with_evidence(EvidenceSpec::FileExists {
            path: path.to_string(),
        })
        .with_source("dyadt-cli");

    let verifier = Verifier::new();
    let report = verifier.verify(&claim);
    print_report(&report);

    verdict_to_exit_code(report.overall_verdict)
}

fn compute_hash(path: &str) -> ExitCode {
    match fs::read(path) {
        Ok(contents) => {
            let mut hasher = Sha256::new();
            hasher.update(&contents);
            let hash = hex::encode(hasher.finalize());
            println!("{}", hash);
            println!("\nEvidence spec:");
            println!(
                r#"{{ "type": "FileWithHash", "spec": {{ "path": "{}", "sha256": "{}" }} }}"#,
                path, hash
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            ExitCode::from(3)
        }
    }
}

fn verify_multiple(path: &str) -> ExitCode {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            return ExitCode::from(3);
        }
    };

    let claims: Vec<Claim> = match serde_json::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing claims JSON: {}", e);
            return ExitCode::from(3);
        }
    };

    let verifier = Verifier::new();
    let mut worst_verdict = Verdict::Confirmed;

    println!("Verification Report");
    println!("===================\n");

    for claim in &claims {
        let report = verifier.verify(claim);
        print_report(&report);
        println!();

        // Track worst verdict
        worst_verdict = match (worst_verdict, report.overall_verdict) {
            (_, Verdict::Refuted) => Verdict::Refuted,
            (Verdict::Refuted, _) => Verdict::Refuted,
            (_, Verdict::Inconclusive) => Verdict::Inconclusive,
            (Verdict::Inconclusive, _) => Verdict::Inconclusive,
            (_, Verdict::Unverifiable) => Verdict::Unverifiable,
            (Verdict::Unverifiable, _) => Verdict::Unverifiable,
            (Verdict::Confirmed, Verdict::Confirmed) => Verdict::Confirmed,
        };
    }

    println!("-------------------");
    println!("Overall: {:?}", worst_verdict);

    verdict_to_exit_code(worst_verdict)
}

fn print_report(report: &VerificationReport) {
    println!("{}", report.summary());

    if let Some(ref source) = report.claim.source {
        println!("  Source: {}", source);
    }

    for result in &report.evidence_results {
        let icon = match result.verdict {
            Verdict::Confirmed => "  ✓",
            Verdict::Refuted => "  ✗",
            Verdict::Inconclusive => "  ?",
            Verdict::Unverifiable => "  ⊘",
        };

        let evidence_desc = match &result.spec {
            EvidenceSpec::FileExists { path } => format!("File exists: {}", path),
            EvidenceSpec::FileWithHash { path, .. } => format!("File hash: {}", path),
            EvidenceSpec::FileContains { path, substring } => {
                format!("File contains '{}': {}", substring, path)
            }
            EvidenceSpec::DirectoryExists { path } => format!("Directory exists: {}", path),
            EvidenceSpec::CommandSucceeds { command, .. } => {
                format!("Command succeeds: {}", command)
            }
            EvidenceSpec::Custom { name, .. } => format!("Custom check: {}", name),
        };

        println!("{} {}", icon, evidence_desc);

        if let Some(ref details) = result.details {
            println!("      {}", details);
        }
    }
}

fn verdict_to_exit_code(verdict: Verdict) -> ExitCode {
    match verdict {
        Verdict::Confirmed => ExitCode::SUCCESS,
        Verdict::Refuted => ExitCode::from(1),
        Verdict::Inconclusive | Verdict::Unverifiable => ExitCode::from(2),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return ExitCode::from(3);
    }

    match args[1].as_str() {
        "check" => {
            if args.len() < 3 {
                eprintln!("Usage: dyadt check <claim.json>");
                ExitCode::from(3)
            } else {
                verify_claim_file(&args[2])
            }
        }
        "verify" => {
            if args.len() < 3 {
                eprintln!("Usage: dyadt verify <path>");
                ExitCode::from(3)
            } else {
                quick_verify(&args[2])
            }
        }
        "hash" => {
            if args.len() < 3 {
                eprintln!("Usage: dyadt hash <file>");
                ExitCode::from(3)
            } else {
                compute_hash(&args[2])
            }
        }
        "report" => {
            if args.len() < 3 {
                eprintln!("Usage: dyadt report <claims.json>");
                ExitCode::from(3)
            } else {
                verify_multiple(&args[2])
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            ExitCode::from(3)
        }
    }
}
