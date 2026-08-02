use super::{age_days, epoch_day, valid_utc};

fn day(year: i64, month: i64, day: i64) -> i64 {
    epoch_day(year, month, day)
}

fn date(y: u8, m: u8, d: u8) -> (i64, i64, i64) {
    (i64::from(y), i64::from(m % 12 + 1), i64::from(d % 28 + 1))
}

fn consecutive_days_increase(y: u8, m: u8, d: u8) -> bool {
    let (year, month, d) = date(y, m, d);
    let (next_year, next_month, next_day) = next(year, month, d);
    day(next_year, next_month, next_day) == day(year, month, d) + 1
}

fn next(year: i64, month: i64, d: i64) -> (i64, i64, i64) {
    let last = days_in_month(year, month);
    if d < last {
        (year, month, d + 1)
    } else if month < 12 {
        (year, month + 1, 1)
    } else {
        (year + 1, 1, 1)
    }
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if leap(year) => 29,
        2 => 28,
        _ => 31,
    }
}

fn leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn age_is_self_zero(value: String) -> bool {
    !valid_utc(&value) || age_days(&value, &value) == Some(0)
}

fn week_rollover_is_seven(d: i64, m: i64) -> bool {
    let d = d % 24;
    let m = m % 12 + 1;
    let start = day(2000, m, d);
    let end = day(2000, m, d + 7);
    end - start == 7
}

fn valid_utc_rejects_garbage(value: String) -> bool {
    !valid_utc(&value) || value.len() == 20
}

#[test]
fn date_math_properties() {
    quickcheck::quickcheck(consecutive_days_increase as fn(u8, u8, u8) -> bool);
    quickcheck::quickcheck(age_is_self_zero as fn(String) -> bool);
    quickcheck::quickcheck(week_rollover_is_seven as fn(i64, i64) -> bool);
    quickcheck::quickcheck(valid_utc_rejects_garbage as fn(String) -> bool);
}

#[test]
fn age_days_across_year_boundary() {
    assert_eq!(
        age_days("2024-12-31T23:59:59Z", "2025-01-01T00:00:00Z"),
        Some(1)
    );
}

#[test]
fn valid_utc_accepts_known_timestamps() {
    assert!(valid_utc("2026-07-17T04:59:27Z"));
    assert!(!valid_utc("2026-13-01T00:00:00Z"));
    assert!(!valid_utc("2026-01-01T24:00:00Z"));
    assert!(!valid_utc("2026-01-01T00:00:61Z"));
}
