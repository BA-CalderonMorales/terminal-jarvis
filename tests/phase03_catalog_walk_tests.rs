mod phase03_fixture;

use phase03_fixture::walk_catalog;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn catalog_walk_records_all_rows_once_without_effects() {
    let report = walk_catalog();
    assert_eq!(report.rows.len(), 225);
    let mut keys = BTreeSet::new();
    let mut harnesses = BTreeSet::new();
    let mut per_harness = BTreeMap::new();
    let mut support = BTreeMap::new();
    let mut effects = BTreeMap::new();
    for row in &report.rows {
        assert!(keys.insert((&row.harness, &row.capability)));
        harnesses.insert(&row.harness);
        *per_harness.entry(&row.harness).or_insert(0) += 1;
        *support.entry(row.support.as_str()).or_insert(0) += 1;
        let effect = row.effect.split(';').next().unwrap();
        *effects.entry(effect).or_insert(0) += 1;
        assert_eq!(row.evidence, "deterministic");
        assert_eq!(row.guard, format!("blocked:{}:exit-4", row.support));
        assert_eq!(row.result, "pass");
        assert!(!row.argv.is_empty());
        assert!(!row.platforms.is_empty());
        assert!(!row.executable.is_empty());
        assert!(!row.source.is_empty());
        assert!(!row.verified_at.is_empty());
        assert!(!row.summary.is_empty());
    }
    assert_eq!(harnesses.len(), 25);
    assert!(per_harness.values().all(|count| *count == 9));
    assert_eq!(
        support,
        BTreeMap::from([("disabled", 23), ("stub", 99), ("unknown", 103)])
    );
    assert_eq!(
        effects,
        BTreeMap::from([
            ("dangerous", 27),
            ("read-only", 123),
            ("state-changing", 75)
        ])
    );
    let rendered = report.tsv();
    assert_eq!(rendered.lines().count(), 226);
    assert!(rendered
        .lines()
        .skip(1)
        .all(|line| line.split('\t').count() == 15));
    if let Some(path) = std::env::var_os("TJ_PHASE03_REPORT_PATH") {
        std::fs::write(path, &rendered).expect("coverage report is writable");
    }
    if std::env::var_os("TJ_PHASE03_PRINT_REPORT").is_some() {
        print!("{rendered}");
    }
}
