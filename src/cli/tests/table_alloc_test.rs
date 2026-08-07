use super::*;

fn with_columns<T>(value: &str, test: impl FnOnce() -> T) -> T {
    let _guard = crate::ENV_LOCK
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let previous = std::env::var_os("COLUMNS");
    std::env::set_var("COLUMNS", value);
    let result = test();
    if let Some(value) = previous {
        std::env::set_var("COLUMNS", value);
    } else {
        std::env::remove_var("COLUMNS");
    }
    result
}

#[test]
fn fit_scales_columns_above_floor_when_over_budget() {
    let mut widths = vec![8, 3];
    fit(&mut widths, 6, &[3, 0]);
    assert_eq!(widths, vec![4, 2]);
}
