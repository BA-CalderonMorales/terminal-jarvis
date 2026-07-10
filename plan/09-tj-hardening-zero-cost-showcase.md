---
id: "09"
target: v0.1.13
title: Zero-Cost Showcase
status: proposed
owner: docs-demo-owner
reviewer: accessibility-reviewer
depends_on: ["07", "08"]
blocks: ["10", "16"]
---

# 09 - Zero-Cost Showcase

## Objective

Ship useful showcase paths that do not place compute cost on the maintainer,
establish demand and feedback, and remain available if hosted infrastructure is
disabled.

## Surfaces

### Static terminal playback

- Generate an asciicast from the canonical fixture walkthrough.
- Host the cast and standalone player as static documentation assets.
- Pin dimensions and test rendering at desktop/mobile widths.
- Regenerate only from a verified release/candidate fixture.

### Killercoda guided scenario

- Use an ephemeral Ubuntu scenario with no production secrets.
- Fetch and verify the exact release asset or fixture package.
- Provide guided steps and verification for list, show, plan, check, security,
  and simulated routing.
- Keep the scenario standalone and clear about simulation and session deletion.

### User-funded full environment

- Keep an optional Codespaces path for users who want a full repository/editor.
- Use the smallest maintainable devcontainer configuration; do not install all
  third-party harnesses or enable owner-funded prebuilds by default.
- Make it clear the visitor's GitHub account/quota owns the environment.

## Research References

- [Asciinema standalone player](https://docs.asciinema.org/manual/player/)
- [Killercoda creator documentation](https://killercoda.com/creators)
- [Killercoda pricing](https://killercoda.com/pricing)
- [GitHub Codespaces](https://github.com/features/codespaces)
- [Codespaces billing ownership](https://docs.github.com/en/codespaces/managing-codespaces-for-your-organization/choosing-who-owns-and-pays-for-codespaces-in-your-organization)

## Work

- [ ] Implement and verify the static recording workflow.
- [ ] Implement and verify the Killercoda scenario from a versioned source repository.
- [ ] Decide and document the minimal Codespaces/devcontainer scope.
- [ ] Add version/drift checks so showcase content cannot silently lag releases.
- [ ] Add accessible captions/transcript and keyboard navigation.
- [ ] Add privacy-safe usage/feedback collection sufficient to judge hosted demand.
- [ ] Prove all three paths work with hosted sandbox services disabled.

## Acceptance Criteria

- [ ] `ZER-01` Static playback works from static hosting with no application backend.
- [ ] `ZER-02` Killercoda starts a clean guided session without maintainer credentials.
- [ ] `ZER-03` Every showcased command uses the page 08 fixture and page 05 claims.
- [ ] `ZER-04` Codespaces is optional, minimal, and not billed to the maintainer by default.
- [ ] `ZER-05` Version drift causes a visible failure or required refresh.
- [ ] `ZER-06` Accessibility and transcript review pass.
- [ ] `ZER-07` These surfaces remain the fallback when the hosted kill switch is active.

## Evidence

| Criterion | Method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| ZER-01 | static deployment test | pending | pending | pending | pending | pending |
| ZER-02 | fresh scenario walkthrough | pending | pending | pending | pending | pending |
| ZER-03 | fixture hash/version check | pending | pending | pending | pending | pending |
| ZER-04 | billing/config review | pending | pending | pending | pending | pending |
| ZER-05 | stale-version simulation | pending | pending | pending | pending | pending |
| ZER-06 | accessibility review | pending | pending | pending | pending | pending |
| ZER-07 | kill-switch fallback test | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: external scenario availability changes. Static playback remains owned and available.
- Risk: devcontainer maintenance expands. Keep it contributor-focused and based on
  standard images/features rather than bundling all agents.
- Rollback trigger: scenario executes unreviewed actions or content drifts from release.
- Rollback action: unpublish the interactive scenario and retain static playback.

## Completion Gate

Complete only when a fresh unauthenticated reviewer can finish the static and
guided journeys and hosted infrastructure is proven unnecessary for them.
