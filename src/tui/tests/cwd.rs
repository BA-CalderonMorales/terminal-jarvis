use super::cwd_label_for;

#[test]
fn cwd_label_roots_at_home() {
    assert_eq!(
        cwd_label_for("/home/caldo/work/terminal-jarvis", Some("/home/caldo")),
        "~/work/terminal-jarvis"
    );
    assert_eq!(
        cwd_label_for("/usr/local/bin", Some("/home/caldo")),
        "/usr/local/bin"
    );
}

#[test]
fn cwd_label_ellipsizes_long_paths_at_component_boundaries() {
    let dotted = cwd_label_for(
        "/home/caldo/world/repositories/working/terminal-jarvis",
        Some("/home/caldo"),
    );
    assert!(dotted.starts_with(".../"));
    assert!(dotted.ends_with("terminal-jarvis"));
    assert!(dotted.chars().count() <= 32);
    assert!(dotted.starts_with(".../working/terminal-jarvis"));
}

#[test]
fn cwd_label_hard_cuts_an_oversized_single_component() {
    let component = "xcargo-mutants-terminal-jarvis-very-long-name.tmp";
    let label = cwd_label_for(&format!("/home/caldo/{component}/sub"), Some("/home/caldo"));
    let keep = 32 - 4 - 1;
    let ellipsized: String = component
        .chars()
        .skip(component.chars().count() - keep)
        .collect();
    assert_eq!(label, format!("…{ellipsized}"));
    assert!(label.chars().count() <= 32);
}
