use crate::logic::preflight_kit::*;
use std::fs;
#[test]
fn metadata_version_mismatch_fails_clearly() {
    let root = make_root("preflight-version-mismatch");
    write_metadata(&root, "0.1.5", "0.1.4", "0.1.5");
    let output = run_preflight(&root, &[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("npm package version 0.1.4 does not match Cargo 0.1.5"));
}

#[test]
fn matching_metadata_passes_without_tag_context() {
    let root = make_root("preflight-metadata-ok");
    write_metadata(&root, "0.1.5", "0.1.5", "0.1.5");
    let output = run_preflight(&root, &[]);
    assert!(output.status.success());
}

#[test]
fn tag_must_match_expected_main_tip() {
    let root = make_root("preflight-main-mismatch");
    write_metadata(&root, "0.1.5", "0.1.5", "0.1.5");
    git(&root, &["init", "-b", "main"]);
    commit(&root, "release metadata");
    git(&root, &["tag", "v0.1.5"]);
    fs::write(root.join("after-tag.txt"), "new main tip\n").unwrap();
    commit(&root, "advance main");
    git(&root, &["checkout", "--detach", "v0.1.5"]);
    let output = run_preflight(&root, &["--tag", "v0.1.5", "--expected-main-ref", "main"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("v0.1.5 points to") && stderr.contains("but main is"));
}
