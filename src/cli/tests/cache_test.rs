use super::*;

fn words(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|part| part.to_string()).collect()
}

fn plain_handle(parts: &[&str]) -> String {
    let previous = crate::cli::logic::style::set(true, true);
    let result = handle(&words(parts)).unwrap();
    crate::cli::logic::style::restore(previous);
    result
}

#[test]
fn empty_and_status_route_to_status() {
    assert!(plain_handle(&[]).contains("distribution:"));
    assert!(plain_handle(&["status"]).contains("distribution:"));
}

#[test]
fn clear_and_refresh_report_maintenance() {
    assert!(plain_handle(&["clear"]).contains("cache clear:"));
    assert!(plain_handle(&["refresh"]).contains("cache refresh:"));
}

#[test]
fn unknown_route_is_a_usage_error() {
    let previous = crate::cli::logic::style::set(true, true);
    assert!(handle(&words(&["bogus"])).is_err());
    crate::cli::logic::style::restore(previous);
}

#[test]
fn maintenance_prefixes_with_the_request() {
    let previous = crate::cli::logic::style::set(true, true);
    let body = super::maintenance("clear", "after terminal-jarvis exits");
    crate::cli::logic::style::restore(previous);
    assert!(body.contains("cache clear:"));
    assert!(body.contains("remove") || body.contains("no wrapper cache"));
}
