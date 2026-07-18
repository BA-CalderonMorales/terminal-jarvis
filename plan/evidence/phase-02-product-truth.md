# Phase 02 Product Truth Evidence

Tested implementation ref: `4ec142fe954cd998df92775a38240a9a523d1547`.

The exact-ref development and staged-package catalog walks are byte-identical,
contain 225 unique rows plus one header, and bind version 0.1.13, catalog digest
`5c1be7ada59d74ba5862f5b32b72ec5ca05b2c77200197f690584bb1e03c28bb`,
and gate digest `6496a7824dadba4548226a868982d470c4f694983a98d35a881b545b6272381b`.
All rows are guarded: 99 stub, 23 disabled, and 103 unknown. No first-class
guarantee is promoted.

The adversarial report records exact commands for diagnostics, lifecycle,
streams, signals, redaction, npm cache integrity/recovery, architecture,
checksum, path shadowing, and support drift. Timeout and debug are explicitly
not applicable because Phase 01 froze no such v0.1.13 product surface.

No network, credential, provider, tag, publication, or registry/tap mutation
was used to produce this evidence.
