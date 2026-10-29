use super::*;
use crate::catalog::logic::freshness_clock::from_day;
#[test]
fn utc_calendar_roundtrips_a_complete_gregorian_cycle() {
    // Every day in a 400-year Gregorian cycle, including century corrections.
    for day in 0..146_097 {
        let timestamp = from_day(day);
        assert!(
            valid_utc(&timestamp),
            "invalid date for epoch day {day}: {timestamp}"
        );
        assert_eq!(date_day(&timestamp), Some(day), "{timestamp}");
    }
}
#[test]
fn century_leap_rules_are_not_the_four_year_shortcut() {
    for timestamp in [
        "1600-02-29T00:00:00Z",
        "2000-02-29T00:00:00Z",
        "2400-02-29T00:00:00Z",
    ] {
        assert!(valid_utc(timestamp), "{timestamp}");
    }
    for timestamp in [
        "1700-02-29T00:00:00Z",
        "1900-02-29T00:00:00Z",
        "2100-02-29T00:00:00Z",
    ] {
        assert!(!valid_utc(timestamp), "{timestamp}");
    }
}

#[test]
fn runtime_utc_date_tracks_the_system_clock_even_across_midnight() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let before = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 86_400;
    let timestamp = crate::catalog::today_utc();
    let after = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 86_400;
    assert!(valid_utc(&timestamp), "invalid runtime date: {timestamp}");
    let day = date_day(&timestamp).unwrap();
    assert!(
        (before as i64..=after as i64).contains(&day),
        "runtime date drifted: {timestamp}"
    );
}
