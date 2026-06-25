# Homebrew Surface

This is the source-build Homebrew surface for the harness-catalog rewrite.

It is intentionally `head`-only until stable release archives and checksums are
available. Release packaging generates versioned formulas under ignored
`dist/` output.

Local checks:

```bash
ruby -c homebrew/Formula/terminal-jarvis.rb
```

Release work must replace the `head`-only formula with versioned URLs and real
SHA-256 checksums.
