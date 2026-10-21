# A workflow job that runs scripts/ checks out the tree

Branch `develop` -> `main`, 2026-10-21. Linked from `README.md`.

**Decision:** The `publish-npm` job checks out the repository at the release tag
before it runs `scripts/bash/release/index.sh`, and `verify` fails when any
workflow job invokes a repository script without `actions/checkout` in that job.

**Context:** The npm publish body moved out of the workflow's `run:` block into
`scripts/bash/release/logic/publish-npm.sh` so the rehearsal could exercise the
shipped bytes. The job still only downloaded a package artifact, so on the
v0.1.21 tag `scripts/bash/release/index.sh` did not exist and the job died with
exit 127. crates.io, the GitHub release, and the Homebrew tap had already
shipped, so the release was live everywhere except npm. No PR gate reads the
CD workflow: it runs only on a tag or `workflow_dispatch`, which is exactly the
run where a missing checkout is expensive.

**Considered:** Inlining the publish body again removes the failure but also
removes the rehearsal that keeps the shipped bytes honest. Running the manual
npm channel workflows would publish the release with `NPM_TOKEN` and the
single-read race those workflows still carry, and would leave the CD job broken
for the next tag. Watching the first release by hand does not make the next one
safer.

**Consequence:** The npm job needs the tree, the same as every other job in
`cd-multiplatform.yml`. `scripts/bash/verify/index.sh workflow-checkout` is part
of `verify`, so a future workflow edit that calls `scripts/` without a checkout
fails before a tag exists. The v0.1.21 recovery used `workflow_dispatch` from
`develop` with the tag pinned, so the published version stayed at `f1e0270`
while the workflow fix could land on `main` afterwards.
