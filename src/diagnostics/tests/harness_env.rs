use super::*;
use crate::contracts::EnvMode;

#[test]
fn ready_requires_presence_by_mode() {
    assert!(ready(EnvMode::None, &[]));
    assert!(!ready(EnvMode::None, &[ValueState::Missing]));
    assert!(ready(EnvMode::Optional, &[ValueState::Missing]) && ready(EnvMode::Optional, &[]));
    assert!(!ready(EnvMode::Any, &[ValueState::Missing]));
    assert!(ready(EnvMode::Any, &[ValueState::Present]));
    assert!(!ready(EnvMode::All, &[]));
    assert!(ready(EnvMode::All, &[ValueState::Present; 2]));
    assert!(!ready(
        EnvMode::All,
        &[ValueState::Present, ValueState::Missing]
    ));
}

#[test]
fn aggregate_maps_state_sets() {
    assert_eq!(aggregate(&[]), Code::Malformed);
    assert_eq!(aggregate(&[ValueState::Malformed]), Code::Malformed);
    assert_eq!(aggregate(&[ValueState::Empty]), Code::Empty);
    assert_eq!(aggregate(&[ValueState::Missing]), Code::Missing);
}
