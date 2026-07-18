# Terminal Jarvis 0.1.13 Evaluation Kit

This kit is an offline, visibly simulated evaluation. It selects only the exact
payload matching one of five allowlisted targets and runs fixed read-only product
queries. It never runs a coding-agent harness, accepts a command, uses a network,
authenticates, installs, updates, uploads, or mutates a repository.

Run `./run.sh` on Linux/macOS or `run.cmd` on Windows. The only accepted option is
`--report`, which prints a sanitized JSON report to standard output. Redirect it
to a file only when you intentionally want to retain it.

Verify `SHA256SUMS` before running. `manifest-v1.json` binds every component to
the candidate ref, version, target, size, and SHA-256. The manifest SHA-256 is the
kit content identity reported by launchers; the separately distributed archive
checksum identifies the ZIP bytes. `component-inventory.spdx.json` is the SBOM.

Repository/ref provenance is recorded in `provenance-v1.json`. Compare its ref
to the reviewed repository commit and its component hashes to `manifest-v1.json`.
Any eventual external attestation must bind that repository, ref, manifest digest,
and exact archive digest. This kit does not create or publish an attestation.
