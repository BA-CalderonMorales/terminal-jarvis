use super::*;

/// An archive or direct download carries no channel metadata. `update`
/// refuses with guidance in that case, so `check` warns rather than fails.
#[test]
fn unknown_update_route_warns_without_failing_the_check() {
    let unknown = RuntimeInput {
        update_route: "unknown".into(),
        ..RuntimeInput::default()
    };
    let (records, ok) = collect(&input(gate_dir(), unknown), &Redactor::new(None, None));
    assert!(ok);
    let route = records.iter().find(|r| r.key == "update.route").unwrap();
    assert_eq!(
        (route.code, route.severity),
        (Code::Unknown, Severity::Warning)
    );
}
