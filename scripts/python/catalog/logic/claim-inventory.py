#!/usr/bin/env python3
"""Generate the machine-readable claim inventory for harnesses/ and gates/.

Every key in every catalog manifest is one machine-readable assertion about
software this repository does not own. This script emits one TSV row per
assertion, so a reader can grep for the weak classes instead of trusting prose:

  subject   harness:<name> | harness:<name>/<capability> | gate:<name>
  field     the manifest key
  kind      what sort of claim the key carries
  value     the asserted value, with tabs and newlines escaped
  held_by   the mechanism, if any, that fails closed when the value is wrong
  basis     evidence | assertion
  verdict   witnessed | gate | inert | frozen | external | asserted

Verdict vocabulary::

  witnessed  a recorded real execution backs the value (support=verified)
  gate       an executable cross-field invariant in the catalog shape gate
             constrains the value
  inert      a gate covers the field, but a higher-precedence guard makes the
             row unreachable, so the value can never be exercised
  frozen     the value is only compared against a compile-time constant, so it
             cannot drift into a measurable failure
  external   provenance confirmable only outside this repository
  asserted   human narration; nothing here executes it

A key the catalog loader accepts but this script does not classify is an
error, so the inventory stays complete as the catalog grows.

--check regenerates and diffs against the committed artifact.
"""
import argparse
import json
import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent.parent.parent.parent
HARNESSES = ROOT / "harnesses"
GATES = ROOT / "gates"
DEFAULT_OUTPUT = ROOT / "docs" / "claim-inventory.tsv"
HEADER = ["subject", "field", "kind", "value", "held_by", "basis", "verdict"]

KIND = {
    "name": "identity",
    "binary": "identity",
    "executable": "identity",
    "display": "narration",
    "description": "narration",
    "summary": "narration",
    "install_hint": "narration",
    "source": "provenance",
    "support": "support-state",
    "evidence": "support-state",
    "effect": "risk-class",
    "network": "risk-class",
    "interaction": "risk-class",
    "platforms": "platform-scope",
    "command": "execution-shape",
    "args": "execution-shape",
    "verified_at": "freshness",
    "env_mode": "environment",
    "env": "environment",
    "package": "install-source",
}

HELD_BY = {
    "name": "gate:identity",
    "binary": "gate:identity",
    "executable": "gate:identity",
    "support": "gate:evidence-pairing",
    "evidence": "gate:evidence-pairing",
    "effect": "gate:effect-class",
    "network": "gate:effect-class",
    "interaction": "gate:effect-class",
    "platforms": "gate:platform-name",
    "command": "gate:command-shape",
    "args": "gate:command-shape",
    "verified_at": "constant:freshness",
    "env_mode": "gate:env",
    "env": "gate:env",
}

EXECUTING = {"verified", "expected"}
BEHAVIOUR = {"execution-shape", "risk-class", "platform-scope", "support-state"}


def clean(value):
    if isinstance(value, list):
        return json.dumps(value, ensure_ascii=True)
    if isinstance(value, bool):
        return "true" if value else "false"
    return str(value).replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n")


def verdict(kind, support, platforms, held_by):
    if kind == "provenance":
        return "external"
    if kind == "freshness":
        return "frozen"
    if kind == "platform-scope":
        if support not in EXECUTING:
            return "inert"
        if not platforms:
            return "asserted"
        return "witnessed" if support == "verified" else "gate"
    if kind == "narration" or held_by == "none":
        return "asserted"
    if support == "verified" and kind in BEHAVIOUR:
        return "witnessed"
    return "gate"


def rows_for(subject, data, support):
    for field in sorted(data):
        try:
            kind = KIND[field]
        except KeyError:
            raise SystemExit(f"claim-inventory: unrecognized manifest key {field!r} in {subject}")
        held_by = HELD_BY.get(field, "none")
        yield [
            subject,
            field,
            kind,
            clean(data[field]),
            held_by,
            "evidence" if held_by != "none" else "assertion",
            verdict(kind, support, data.get("platforms", []), held_by),
        ]


def manifests():
    for harness in sorted(path for path in HARNESSES.iterdir() if path.is_dir()):
        facade = harness / "index.toml"
        if facade.is_file():
            yield f"harness:{harness.name}", load(facade), None
        for facet in sorted(path for path in harness.iterdir() if path.is_dir()):
            index = facet / "index.toml"
            if index.is_file():
                data = load(index)
                yield f"harness:{harness.name}/{facet.name}", data, data.get("support")
    for gate in sorted(path for path in GATES.iterdir() if path.is_dir()):
        index = gate / "index.toml"
        if index.is_file():
            yield f"gate:{gate.name}", load(index), None


def load(path):
    with path.open("rb") as stream:
        return tomllib.load(stream)


def build():
    rows = []
    for subject, data, support in manifests():
        rows.extend(rows_for(subject, data, support))
    rows.sort(key=lambda row: (row[0], row[1]))
    return rows


def report(rows):
    # Bytes, and LF only: text mode would translate to CRLF on Windows and the
    # committed artifact would stop matching byte-for-byte across hosts.
    lines = ["\t".join(HEADER)]
    lines.extend("\t".join(row) for row in rows)
    return ("\n".join(lines) + "\n").encode("utf-8")


def tally(rows):
    counts = {}
    for row in rows:
        counts[row[6]] = counts.get(row[6], 0) + 1
    return counts


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--output", default=str(DEFAULT_OUTPUT), help="destination TSV path")
    parser.add_argument("--check", action="store_true", help="fail when the artifact is stale")
    args = parser.parse_args()

    rows = build()
    rendered = report(rows)
    output = pathlib.Path(args.output)
    summary = ", ".join(f"{k}={v}" for k, v in sorted(tally(rows).items()))

    if args.check:
        current = output.read_bytes() if output.is_file() else b""
        if current != rendered:
            print(
                "claim-inventory: artifact is stale; regenerate with "
                "scripts/python/catalog/index.py claim-inventory",
                file=sys.stderr,
            )
            return 1
        print(f"claim-inventory matches the catalog ({len(rows)} claims: {summary})")
        return 0

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(rendered)
    print(f"claim-inventory: wrote {len(rows)} claims to {output}")
    print(f"claim-inventory: verdicts {summary}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
