# One npm publisher that waits for the registry

Branch `release/0.1.22` -> `develop`, 2026-10-21. Linked from `README.md`.

**Decision:** The tag job and the stable, beta, and recovery workflows all call
`scripts/bash/release/logic/publish-npm.sh`. The publisher polls until the
registry serves the version and every dist-tag (600 seconds by default),
supports a single channel and `--dry-run`, and refuses a version that does not
match `Cargo.toml`. The tag job keeps `NPM_TOKEN` as a fallback until a Trusted
Publisher is registered on npmjs.com, then the token is removed and OIDC is the
only path.

**Context:** v0.1.21 exposed three release-only failures in one night. The npm
job called a repository script with no checkout (exit 127). The OIDC exchange
returned `ENEEDAUTH` because the Trusted Publisher did not exist (exit 1). Each
manual channel workflow still published and then read the registry once, so the
successful publish and both retags were recorded as failures while npm was
still processing the write. None of these could fail in a pull request; the CD
workflow runs only on a tag or dispatch, which is the run where the cost is
highest.

**Considered:** Keeping the manual workflows inline preserves their separate
inputs but also preserves the copy that drifts from the shipped publisher and
the immediate read that misreports success. Requiring OIDC before the next tag
is the clean end state, but it blocks the release on a setting only the npm web
UI can create. Publishing only with `NPM_TOKEN` is solid today and gives up the
short-lived credential the previous branch added. Leaving the 180-second window
in place was rejected: npm's post-publish processing took several minutes for
this package.

**Consequence:** A channel outcome is the registry's served state, not a step's
colour. Manual reruns take the retag path when the version already exists, so a
partial release is resumable. The rehearsal proves publish, retag, single-tag,
dry-run, and the toolchain matrix against a fake registry before a tag exists,
and `workflow-checkout` keeps a job from calling `scripts/` without the tree.
The token in `cd-multiplatform.yml` is a named transitional fallback: remove it
when the Trusted Publisher is registered, and update this record.
