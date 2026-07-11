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

Ship one self-contained showcase that places no compute cost on the maintainer,
then optionally publish external guided/user-funded paths without making them a
release dependency. The static artifact remains available when every external
service and hosted provider is disabled.

## Surfaces

### Static terminal playback

- Generate an asciicast from the canonical fixture walkthrough.
- Vendor or checksum-pin every player asset; do not require a third-party CDN.
- Open the artifact directly or through a loopback static server without an
  application backend, provider account, analytics, or network request.
- GitHub Pages is an optional `OI-PUBLISH` target while public-repository Pages
  and standard Actions remain non-billable; local playback is the release gate.
- Pin dimensions and test rendering at desktop/mobile widths.
- Regenerate only from a verified release/candidate fixture.

### Killercoda guided scenario

- Treat Killercoda as `external-free-tier`, not as the zero-host baseline. At
  research time public scenarios are free, but the service requires a creator
  account, repository deploy key/webhook, mutable platform images, and user limits.
- Require `OI-PUBLISH`; otherwise record `not-selected` and do not create an
  account, deploy key, webhook, repository integration, or scenario.
- Use an ephemeral Ubuntu scenario with no production secrets.
- Fetch and verify the exact release asset or fixture package.
- Provide guided steps and verification for list, show, plan, check, security,
  and simulated routing.
- Keep the scenario standalone and clear about simulation and session deletion.

### User-funded full environment

- Treat Codespaces as optional `user-metered`, never as free compute. Require
  `OI-USER-METERED` before displaying a link.
- Use the smallest maintainable devcontainer configuration; do not install all
  third-party harnesses or configure owner-funded prebuilds.
- Make the visitor acknowledge that their GitHub account/quota owns the
  environment and may block or bill usage after included quota is exhausted.

## Research References

- [Asciinema standalone player](https://docs.asciinema.org/manual/player/)
- [Killercoda creator documentation](https://killercoda.com/creators)
- [Killercoda pricing](https://killercoda.com/pricing)
- [GitHub Codespaces](https://github.com/features/codespaces)
- [Codespaces billing ownership](https://docs.github.com/en/codespaces/managing-codespaces-for-your-organization/choosing-who-owns-and-pays-for-codespaces-in-your-organization)
- [GitHub Pages availability](https://docs.github.com/en/pages/getting-started-with-github-pages)
- [GitHub Pages limits](https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits)
- [GitHub Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions)

## Work

- [ ] Implement and verify the static recording workflow.
- [ ] Verify a fully offline local/loopback preview with vendored or pinned assets.
- [ ] Record Killercoda as `not-selected`, or obtain `OI-PUBLISH` and verify the
  scenario from a versioned source repository without maintainer credentials.
- [ ] Record Codespaces as `not-selected`, or obtain `OI-USER-METERED` and
  document a minimal devcontainer with no owner-funded prebuild.
- [ ] Add version/drift checks so showcase content cannot silently lag releases.
- [ ] Add accessible captions/transcript and keyboard navigation.
- [ ] Use a repository issue/discussion link or no feedback collection by default;
  any analytics/feedback SaaS requires `OI-PUBLISH` and a data-retention review.
- [ ] Prove static playback works when GitHub Pages, Killercoda, Codespaces,
  analytics, network access, and both hosted sandbox services are unavailable.

## Acceptance Criteria

- [ ] `ZER-01` Self-contained static playback and transcript work locally with no
  backend, network request, analytics, provider SDK, account, or credential.
- [ ] `ZER-02` Killercoda has reviewed `not-selected` evidence, or its approved
  scenario starts cleanly without maintainer credentials and can be unlinked.
- [ ] `ZER-03` Every showcased command uses the page 08 fixture and page 05 claims.
- [ ] `ZER-04` Codespaces is `not-selected`, or is minimal, user-metered,
  explicitly disclosed, and has no maintainer-funded prebuild or sponsorship.
- [ ] `ZER-05` Version drift causes a visible failure or required refresh.
- [ ] `ZER-06` Accessibility and transcript review pass.
- [ ] `ZER-07` Static playback alone remains the fallback when every hosted and
  optional external surface is disabled.
- [ ] `ZER-08` External outage/absence testing proves no optional link or service
  is required to complete the canonical walkthrough.

## Evidence

| Criterion | Method | Artifact/URL | Ref | UTC | Result | Reviewer |
|---|---|---|---|---|---|---|
| ZER-01 | offline local static walkthrough | pending | pending | pending | pending | pending |
| ZER-02 | selection record and optional scenario walkthrough | pending | pending | pending | pending | pending |
| ZER-03 | fixture hash/version check | pending | pending | pending | pending | pending |
| ZER-04 | billing/config review | pending | pending | pending | pending | pending |
| ZER-05 | stale-version simulation | pending | pending | pending | pending | pending |
| ZER-06 | accessibility review | pending | pending | pending | pending | pending |
| ZER-07 | kill-switch fallback test | pending | pending | pending | pending | pending |
| ZER-08 | all-external-surfaces-unavailable drill | pending | pending | pending | pending | pending |

## Risks and Rollback

- Risk: external scenario availability changes. Static playback remains owned and available.
- Risk: devcontainer maintenance expands. Keep it contributor-focused and based on
  standard images/features rather than bundling all agents.
- Rollback trigger: scenario executes unreviewed actions or content drifts from release.
- Rollback action: unpublish the interactive scenario and retain static playback.

## Completion Gate

Complete when a fresh reviewer can finish the self-contained static journey,
all external services are proven unnecessary, and Killercoda/Codespaces each
have reviewed selected or `not-selected` evidence. Publishing is separate.
