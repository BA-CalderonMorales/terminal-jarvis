# Make install sources auditable and removal honest

**Decision:** Every harness's install channel is checked against its upstream
project, the corrected rows carry the upstream URL as their `source`, and
removal prefers a declared tool-owned uninstaller over a guessed
package-manager inverse. A removal that leaves the binary on PATH is reported
as incomplete, never as success.

**Context:** `cursor-agent` installed `cursor-agent-tools` from PyPI -- a
third-party package, not Cursor's CLI -- while presenting itself as Cursor.
`pi` installed without the `--ignore-scripts` flag its docs require. And
`uninstall` derived `npm uninstall -g <package>` for every npm row, so a tool
installed by its own script reported success while the binary stayed.

**Considered:** Removing the `package` field for script installers was
rejected: it is the advisory surface. Hardcoding per-tool removal commands in
Rust was rejected: the data plane owns per-tool facts. Reporting "no uninstaller
derivable" for every script install was rejected as unhelpful when several
tools document their own remover.

**Consequence:** `download/index.toml` may carry `uninstall_command` +
`uninstall_args` as a pair; the metadata parser rejects a lone key or the pair
on any other capability. `derive_uninstall` tries the declared plan, then npm,
uv, or cargo. `finish_uninstall` re-resolves the binary and fails with
`uninstall_incomplete` when another channel still owns it. `docs/harness-sources.md`
is the audit record; a new or changed harness install row must add its evidence
there. Script installers stay `unknown` support -- they are not deterministically
verifiable, and claiming otherwise is what let the wrong Cursor package in.
