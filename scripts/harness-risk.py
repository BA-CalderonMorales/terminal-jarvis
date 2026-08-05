#!/usr/bin/env python3
"""Aggregate a deterministic risk score per harness from its catalog manifests.

The score is data-driven from harness/*/index.toml so it is stable across
runs and CI environments: a harness's risk is derived from what its
capabilities actually declare (effect, network reach), not opinion.

The score measures destructive and networked surface only:
  +3 per effect = "dangerous"        capability bypasses safeguards (yolo)
  +2 per effect = "state-changing"   capability modifies the system
  +1 per network = true              capability reaches the network

Support evidence (unknown/stub) and empty platform claims are reported as
catalog-wide facts, not scored: they hold for nearly every harness, so
scoring them turns the report into uniform noise instead of a ranking.

Risk bands: LOW <=8, MEDIUM <=12, HIGH <=16, CRITICAL >16.
--check exits nonzero when MAX overall harness score exceeds the band.

Optionally scans the npm wrapper's dependency graph with vlt (when vlt is
on PATH) to fold a free supply-chain signal into the report. vlt is never
required; without it the report emits the manifest-derived score only.
"""
import argparse
import json
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HARNESSES = ROOT / "harnesses"
NPM_WRAPPER = ROOT / "npm" / "terminal-jarvis"

BANDS = {"low": 8, "medium": 12, "high": 16, "critical": 16}
WEIGHTS = {"dangerous": 3, "state-changing": 2, "network": 1}


def score_capability(data):
    score = 0
    effects = []
    effect = data.get("effect")
    if effect == "dangerous":
        score += WEIGHTS["dangerous"]
        effects.append("dangerous")
    elif effect == "state-changing":
        score += WEIGHTS["state-changing"]
        effects.append("state-changing")
    if data.get("network"):
        score += WEIGHTS["network"]
        effects.append("network")
    return score, effects


def load_index(path):
    with path.open("rb") as stream:
        return tomllib.load(stream)


def risk_band(score):
    if score > BANDS["critical"]:
        return "CRITICAL"
    if score > BANDS["high"]:
        return "HIGH"
    if score > BANDS["medium"]:
        return "MEDIUM"
    return "LOW"


def scan_harness(harness):
    caps = sorted(path for path in harness.iterdir()
                  if path.is_dir() and (path / "index.toml").is_file())
    score = 0
    effects = set()
    details = {}
    for cap in caps:
        data = load_index(cap / "index.toml")
        cap_score, cap_effects = score_capability(data)
        if cap_score:
            details[cap.name] = {"score": cap_score, "effects": cap_effects}
            effects.update(cap_effects)
            score += cap_score
    return score, risk_band(score), sorted(effects), details


def catalog_support(path):
    counts = {}
    for index in path.rglob("index.toml"):
        support = load_index(index).get("support")
        if support:
            counts[support] = counts.get(support, 0) + 1
    return counts


def vlt_signal():
    if shutil.which("vlt") is None:
        return {"available": False, "detail": "vlt not installed; supply-chain signal skipped"}
    lockfile = NPM_WRAPPER / "package-lock.json"
    if not lockfile.is_file():
        return {"available": True, "detail": "npm wrapper package-lock.json missing"}
    result = subprocess.run(
        ["vlt", "query", ":malware", ":vulnerable", ":deprecated"],
        capture_output=True, text=True, cwd=NPM_WRAPPER, timeout=120,
    )
    return {
        "available": True,
        "detail": (result.stdout or result.stderr).strip(),
        "status": result.returncode,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", action="store_true", help="emit a JSON report")
    parser.add_argument("--check", choices=sorted(BANDS), help="fail when the max harness score exceeds this band")
    parser.add_argument("--no-vlt", action="store_true", help="skip the optional vlt supply-chain scan")
    args = parser.parse_args()

    rows = []
    support_counts = {}
    for path in sorted(HARNESSES.iterdir()):
        if not path.is_dir() or not (path / "index.toml").is_file():
            continue
        index = load_index(path / "index.toml")
        score, band, effects, details = scan_harness(path)
        for support, count in catalog_support(path).items():
            support_counts[support] = support_counts.get(support, 0) + count
        rows.append({
            "harness": index.get("name") or path.name,
            "score": score,
            "band": band,
            "effects": effects,
            "capabilities": details,
        })
    rows.sort(key=lambda row: -row["score"])
    max_score = max((row["score"] for row in rows), default=0)
    max_band = risk_band(max_score)

    signal = {"available": False, "detail": "vlt scan disabled"} if args.no_vlt else vlt_signal()

    if args.json:
        print(json.dumps({
            "harnesses": rows,
            "max_score": max_score,
            "max_band": max_band,
            "support_evidence": support_counts,
            "vlt": signal,
        }, indent=2))
    else:
        width = max(len(row["harness"]) for row in rows) if rows else 10
        print(f"harness-risk: {len(rows)} harnesses | max {max_score} ({max_band})")
        print(f"{'harness':<{width}}  {'score':>5}  band       effects")
        for row in rows:
            print(f"{row['harness']:<{width}}  {row['score']:>5}  {row['band']:<10} {','.join(row['effects'])}")
        if support_counts:
            print("support evidence (catalog-wide): "
                  + ", ".join(f"{k}={v}" for k, v in sorted(support_counts.items())))
        if signal["available"]:
            print(f"vlt supply-chain: {'ok' if signal.get('status') == 0 else 'findings'}")
            print(signal["detail"])
        else:
            print(f"vlt supply-chain: {signal['detail']}")

    if args.check and max_score > BANDS[args.check]:
        print(f"harness-risk: FAIL max harness score {max_score} exceeds {args.check} band", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
