# Make the release proofs and the npm publisher fail closed

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** `claim-earned` and `windows-clippy` require the pre-fix half to
actually fail and the working tree to pass, and both are executable through the
verify index. The npm publisher requires a dist-tag-capable client (11.21.0+ in
the 11.x line, 12.2.0+ in the 12.x line), CI installs one before it publishes,
and `docs/development.md` records the Trusted Publisher's separate,
default-off `Allow npm dist-tag` permission.

**Context:** Both proofs printed their two halves and then `ok` regardless of
what the halves said, so a baseline that already rejected the stand-in -- or a
host where both trees pass strict clippy -- was recorded as an
old-fails/new-passes contrast. The npm job's minimum (11.5.1) covered OIDC
publishing but not the tokenless `dist-tag add` calls the same job makes, and
the dist-tag permission is off by default: either one ships a release that
publishes and then cannot move `latest`/`stable`/`beta`.

**Considered:** Printing the halves and leaving the verdict to the reader is the
pattern that let both scripts report `ok` for a contrast they never showed.
Raising the npm floor with a plain `sort -V` compare cannot express "11.21.0+ in
11.x, 12.2.0+ in 12.x". Documenting the permission without asserting the client
version leaves the half the job can actually check unasserted.

**Consequence:** A proof that cannot show its contrast is a failure, not a
verdict -- `windows-clippy` now fails on Linux with `there is no warning to
deny`, so it belongs on the host where the pre-fix tree fails. The npm job stops
before the registry when its client cannot retag, and the rehearsal in
`scripts/bash/verify/index.sh publish-rehearsal` exercises the same bytes
against a fake registry.
