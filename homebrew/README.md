# Homebrew Surface

This directory keeps the local source-build Homebrew smoke surface. The public
tap at `BA-CalderonMorales/homebrew-terminal-jarvis` is the binary installer
path and points at versioned GitHub Release archives with checksums.

Both formula surfaces install the harness and optional gate catalogs into
`pkgshare`, so the data-driven command and security-gate behavior stays aligned
with Cargo and npm installs.

Release packaging generates versioned binary formulas under the configured
package output directory, and the release workflow writes that formula into the
tap.

Local checks:

```bash
ruby -c homebrew/Formula/terminal-jarvis.rb
```

Run `scripts/local-cd.sh` to generate and verify versioned archive checksums
before promoting a release formula.
