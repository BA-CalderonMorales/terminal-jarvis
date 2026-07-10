use super::*;
use crate::gates::Gate;

fn gate() -> Gate {
    Gate {
        name: "trivy".to_string(),
        display: "Trivy".to_string(),
        description: "Scan the working tree".to_string(),
        binary: "trivy".to_string(),
        args: vec!["fs".to_string(), ".".to_string()],
        install_hint: "install trivy".to_string(),
    }
}

fn plain<T>(render: impl FnOnce() -> T) -> T {
    let previous = super::super::super::style::set(true, true);
    let result = render();
    super::super::super::style::restore(previous);
    result
}

#[test]
fn rich_gate_output_covers_each_status() {
    let gate = gate();
    assert!(disabled_status("trivy").contains("Security Gate"));
    assert!(configured(&gate, "config", "found").contains("COMMAND"));
    assert!(list(std::slice::from_ref(&gate)).contains("Available Security Gates"));
    assert!(enabled("trivy").contains("Security gate enabled"));
    assert!(disabled().contains("Security gate disabled"));
    assert!(run_result("trivy", 0, "clean").contains("passed"));
    assert!(run_result("trivy", 1, "findings").contains("blocked"));
}

#[test]
fn plain_gate_output_stays_line_oriented() {
    let gate = gate();
    let (status, configured, listed, enabled, disabled, passed, blocked) = plain(|| {
        (
            disabled_status("trivy"),
            configured(&gate, "environment", "missing"),
            list(std::slice::from_ref(&gate)),
            enabled("trivy"),
            super::disabled(),
            run_result("trivy", 0, "clean"),
            run_result("trivy", 1, "findings"),
        )
    });
    assert_eq!(status, "gate: disabled\navailable: trivy\n");
    assert!(configured.contains("gate: trivy (environment)"));
    assert_eq!(listed, "trivy - Scan the working tree\n");
    assert!(enabled.contains("enabled"));
    assert_eq!(disabled, "gate: disabled\n");
    assert!(passed.contains("passed") && blocked.contains("blocked"));
}
