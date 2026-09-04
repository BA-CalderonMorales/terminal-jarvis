use super::nonempty_env;

#[test]
fn environment_values_must_contain_non_whitespace() {
    let key = format!("TJ_ENV_PROBE_{}", std::process::id());
    std::env::set_var(&key, "");
    assert!(!nonempty_env(&key));
    std::env::set_var(&key, " \t ");
    assert!(!nonempty_env(&key));
    std::env::set_var(&key, "ready");
    assert!(nonempty_env(&key));
    std::env::remove_var(key);
}
