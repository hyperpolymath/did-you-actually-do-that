; SPDX-License-Identifier: MPL-2.0
; STATE.scm - Current project state for did-you-actually-do-that

(define state
  '((metadata
      (version . "0.1.0")
      (schema-version . "1.0")
      (created . "2026-01-17")
      (updated . "2026-01-17")
      (project . "did-you-actually-do-that")
      (repo . "https://github.com/hyperpolymath/did-you-actually-do-that"))

    (project-context
      (name . "Did You Actually Do That?")
      (tagline . "Verification framework for validating claimed AI actions against actual outcomes")
      (tech-stack . (rust serde sha2 chrono)))

    (current-position
      (phase . "mvp")
      (overall-completion . 70)
      (components
        (core-library . 90)
        (cli-tool . 85)
        (evidence-types . 75)
        (custom-checkers . 80)
        (tests . 30)
        (ci-cd . 0)
        (documentation . 70))
      (working-features
        "Claim creation and serialization"
        "File existence verification"
        "File hash verification"
        "File content substring matching"
        "Directory existence verification"
        "Command execution verification"
        "Custom checker registration"
        "CLI with check, verify, hash, report commands"
        "JSON claim loading with auto-generated id/timestamp"))

    (route-to-mvp
      (milestone-1
        (name . "Core Complete")
        (status . "done")
        (items
          "Define Claim, Evidence, Verdict types"
          "Implement Verifier with standard checkers"
          "CLI tool with basic commands"))
      (milestone-2
        (name . "Production Ready")
        (status . "in-progress")
        (items
          "Add LICENSE file"
          "Add comprehensive tests"
          "Add CI/CD workflows"
          "Add SECURITY.md and CONTRIBUTING.md"))
      (milestone-3
        (name . "Extended Features")
        (status . "planned")
        (items
          "HTTP reachability checker"
          "Regex content matching"
          "JSON path verification"
          "Async verification support"
          "Publish to crates.io")))

    (blockers-and-issues
      (critical . ())
      (high
        ("No LICENSE file despite MPL-2.0 declaration"))
      (medium
        ("proptest dependency unused"
         "No CI workflows"))
      (low
        ("GitLab token expired - cannot mirror")))

    (critical-next-actions
      (immediate
        "Add LICENSE file"
        "Add SECURITY.md"
        "Add CONTRIBUTING.md")
      (this-week
        "Add property-based tests"
        "Add CI workflows")
      (this-month
        "Publish to crates.io"
        "Add HTTP evidence type"))

    (session-history
      ((date . "2026-01-17")
       (accomplishments
         "Initial project creation"
         "Core library implementation"
         "CLI tool implementation"
         "Fixed JSON deserialization for optional id/timestamp"
         "Added .gitignore"
         "Pushed to GitHub")))))

; Helper functions
(define (get-completion-percentage state)
  (cdr (assoc 'overall-completion (cdr (assoc 'current-position state)))))

(define (get-blockers state priority)
  (cdr (assoc priority (cdr (assoc 'blockers-and-issues state)))))

(define (get-milestone state name)
  (let ((milestones (cdr (assoc 'route-to-mvp state))))
    (find (lambda (m) (equal? (cdr (assoc 'name (cdr m))) name)) milestones)))
