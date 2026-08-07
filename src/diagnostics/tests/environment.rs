use super::*;

#[test]
fn states_have_stable_labels() {
    assert_eq!(ValueState::Missing.as_str(), "missing");
    assert_eq!(ValueState::Empty.as_str(), "empty");
    assert_eq!(ValueState::Malformed.as_str(), "malformed");
    assert_eq!(ValueState::Present.as_str(), "present");
}

#[test]
fn remove_sets_the_state_back_to_missing() {
    let mut environment = Environment::default();
    environment.insert("KEY", "value");
    assert_eq!(environment.state("KEY"), ValueState::Present);
    environment.remove("KEY");
    assert_eq!(environment.state("KEY"), ValueState::Missing);
}
