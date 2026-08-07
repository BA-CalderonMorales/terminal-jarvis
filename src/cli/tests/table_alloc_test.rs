use super::*;

#[test]
fn fit_scales_columns_above_floor_when_over_budget() {
    let mut widths = vec![8, 3];
    fit(&mut widths, 6, &[3, 0]);
    assert_eq!(widths, vec![4, 2]);
}
