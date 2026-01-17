; SPDX-License-Identifier: MPL-2.0
; ECOSYSTEM.scm - Project's place in the broader ecosystem

(ecosystem
  (version . "1.0")
  (name . "did-you-actually-do-that")
  (type . "verification-framework")
  (purpose . "Validate claimed AI actions against observable outcomes")

  (position-in-ecosystem
    (domain . "AI accountability and trust")
    (layer . "verification-infrastructure")
    (role . "claim-evidence-verdict pipeline for action verification"))

  (related-projects
    (sibling-standard
      (maa-framework
        (relationship . "sibling-standard")
        (description . "Mutually Assured Accountability patterns - philosophical foundation")
        (url . "https://gitlab.com/hyperpolymath/maa-framework")
        (integration . "dyadt implements MAA verification principles")))

    (potential-consumer
      (conative-gating
        (relationship . "potential-consumer")
        (description . "AI policy enforcement architecture")
        (url . "https://gitlab.com/hyperpolymath/conative-gating")
        (integration . "Could use dyadt to verify gating decisions were enforced"))

      (claude-code
        (relationship . "potential-consumer")
        (description . "Anthropic CLI tool for AI-assisted coding")
        (integration . "Could verify Claude's claimed file operations"))

      (agentic-scm
        (relationship . "potential-consumer")
        (description . "Agentic source control management")
        (url . "https://gitlab.com/hyperpolymath/agentic-scm")
        (integration . "Could verify agent-performed git operations")))

    (inspiration
      (contract-testing
        (relationship . "inspiration")
        (description . "Consumer-driven contract testing patterns")
        (influence . "Claims are like contracts between AI and user"))

      (property-based-testing
        (relationship . "inspiration")
        (description . "QuickCheck/proptest style testing")
        (influence . "Evidence types are like property generators"))))

  (what-this-is
    "A library and CLI for verifying that claimed actions actually happened"
    "A trust-but-verify mechanism for AI agent outputs"
    "A systematic way to check observable evidence against assertions"
    "A tool for building accountability into AI workflows")

  (what-this-is-not
    "Not a testing framework (though it can be used in tests)"
    "Not an AI agent itself"
    "Not a monitoring or observability system"
    "Not a replacement for proper error handling in AI systems"
    "Not a guarantee of correctness (only verifies observable artifacts)"))
