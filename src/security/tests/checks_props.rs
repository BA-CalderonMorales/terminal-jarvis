use super::missing_env;
use crate::contracts::{EnvMode, Harness};

fn missing_env_none_is_empty(env: Vec<String>) -> bool {
    let harness = Harness {
        name: "h".into(),
        display: "h".into(),
        description: "h".into(),
        binary: "h".into(),
        env_mode: EnvMode::None,
        env,
        capabilities: Vec::new(),
    };
    missing_env(&harness).is_empty()
}

#[test]
fn checks_properties() {
    quickcheck::quickcheck(missing_env_none_is_empty as fn(Vec<String>) -> bool);
}
