# Homebrew Surface

This is the source-build Homebrew surface for the harness-catalog rewrite.

It is intentionally `head`-only until stable release archives and checksums are
available. Release packaging generates versioned formulas under the configured
package output directory.

Local checks:

```bash
ruby -c homebrew/Formula/terminal-jarvis.rb
```

Run `scripts/local-cd.sh` to generate and verify versioned archive checksums
before promoting a release formula.
