; SPDX-License-Identifier: MPL-2.0
; META.scm - Architecture decisions and development practices

(define meta
  '((architecture-decisions

      (adr-001
        (title . "Use Rust for implementation")
        (status . "accepted")
        (date . "2026-01-17")
        (context . "Need a language that provides strong type safety, good performance,
                    and easy distribution as a CLI tool")
        (decision . "Implement in Rust with serde for serialization")
        (consequences
          (positive
            "Type-safe verification logic"
            "Single binary distribution"
            "Good ecosystem for CLI tools"
            "Easy cross-compilation")
          (negative
            "Steeper learning curve for contributors"
            "Longer compile times")))

      (adr-002
        (title . "Tagged enum for evidence types")
        (status . "accepted")
        (date . "2026-01-17")
        (context . "Need to support multiple evidence types with different parameters")
        (decision . "Use serde tagged enum with 'type' and 'spec' fields for JSON compatibility")
        (consequences
          (positive
            "Clear JSON structure"
            "Exhaustive pattern matching in Rust"
            "Easy to add new evidence types")
          (negative
            "Slightly verbose JSON format")))

      (adr-003
        (title . "Custom checker extensibility via closures")
        (status . "accepted")
        (date . "2026-01-17")
        (context . "Users need to add domain-specific verification logic")
        (decision . "Allow registering custom checkers as closures with HashMap<String, String> params")
        (consequences
          (positive
            "Flexible extension mechanism"
            "No trait implementation required")
          (negative
            "Cannot serialize custom checkers"
            "Custom checkers not available in CLI without code changes")))

      (adr-004
        (title . "Exit codes for CI integration")
        (status . "accepted")
        (date . "2026-01-17")
        (context . "Tool should integrate well with CI/CD pipelines")
        (decision . "Use distinct exit codes: 0=Confirmed, 1=Refuted, 2=Inconclusive, 3=Error")
        (consequences
          (positive
            "Easy CI integration"
            "Scripts can check specific failure modes")
          (negative
            "Limited to 4 states without parsing output"))))

    (development-practices
      (code-style
        (formatter . "rustfmt")
        (linter . "clippy")
        (edition . "2021"))
      (security
        (audit-tool . "cargo-audit")
        (hash-algorithm . "SHA-256")
        (no-unsafe . #t))
      (testing
        (unit-tests . "cargo test")
        (property-tests . "proptest")
        (coverage-tool . "cargo-tarpaulin"))
      (versioning . "semver")
      (documentation
        (api-docs . "rustdoc")
        (readme . "markdown"))
      (branching
        (main . "main")
        (features . "feat/*")
        (fixes . "fix/*")))

    (design-rationale
      (why-claims-not-assertions
        "Claims are made by external systems (AI agents) that may be incorrect.
         Assertions imply programmer certainty. Claims acknowledge uncertainty
         and the need for verification.")
      (why-evidence-not-proofs
        "Evidence can be checked but doesn't guarantee truth. A file existing
         doesn't prove an AI created it correctly. We verify observable artifacts,
         not intent or correctness.")
      (why-verdicts-not-booleans
        "Reality is messier than true/false. Inconclusive means insufficient
         evidence. Unverifiable means we can't check. These distinctions matter
         for proper handling of edge cases.")
      (why-sync-not-async
        "Initial implementation is synchronous for simplicity. Most checks
         (file existence, hashing) are fast. Async can be added later for
         network checks without breaking the API."))))
