use super::*;

#[test]
fn override_wins_and_empty_means_embedded() {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = env::var_os(env_const::FLAGS);
    env::set_var(env_const::FLAGS, "");
    assert_eq!(flags_root(), PathBuf::from(env_const::EMBEDDED_FLAGS));
    env::set_var(env_const::FLAGS, "/custom/flags");
    assert_eq!(flags_root(), PathBuf::from("/custom/flags"));
    match previous {
        Some(value) => env::set_var(env_const::FLAGS, value),
        None => env::remove_var(env_const::FLAGS),
    }
}
