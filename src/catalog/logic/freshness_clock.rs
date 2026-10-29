//! UTC dates from the runtime clock; no build-time freshness override.
use std::time::{SystemTime, UNIX_EPOCH};

pub fn today_utc() -> String {
    let Ok(elapsed) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return String::new();
    };
    from_day((elapsed.as_secs() / 86_400) as i64)
}

pub(super) fn from_day(day: i64) -> String {
    let z = day + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let y = year + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}T00:00:00Z")
}

pub fn valid_date(value: &str) -> bool {
    let Ok(year) = value[..4].parse::<u32>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u8>() else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let last = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    (1..=last).contains(&day)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calendar_roundtrips_epoch_and_leap_days() {
        assert_eq!(from_day(0), "1970-01-01T00:00:00Z");
        assert_eq!(from_day(19_782), "2024-02-29T00:00:00Z");
        assert!(valid_date("2024-02-29T00:00:00Z"));
        assert!(!valid_date("2025-02-29T00:00:00Z"));
        assert!(!valid_date("2026-04-31T00:00:00Z"));
    }
}
