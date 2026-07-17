use crate::contracts::{CapabilityPlan, SupportState};

const AS_OF: &str = match option_env!("TERMINAL_JARVIS_EVIDENCE_AS_OF") {
    Some(value) => value,
    None => "2026-07-17T04:59:27Z",
};

pub fn valid_utc(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return false;
    }
    if bytes
        .iter()
        .enumerate()
        .any(|(index, byte)| ![4, 7, 10, 13, 16, 19].contains(&index) && !byte.is_ascii_digit())
    {
        return false;
    }
    valid_range(value, 5, 7, 1, 12)
        && valid_range(value, 8, 10, 1, 31)
        && valid_range(value, 11, 13, 0, 23)
        && valid_range(value, 14, 16, 0, 59)
        && valid_range(value, 17, 19, 0, 59)
}

pub fn status(plan: &CapabilityPlan) -> &'static str {
    status_at(plan, AS_OF)
}

fn status_at(plan: &CapabilityPlan, as_of: &str) -> &'static str {
    let max_age = match plan.support {
        SupportState::Verified => 30,
        SupportState::Expected | SupportState::Manual => 90,
        _ => return "policy-reviewed",
    };
    match age_days(&plan.verified_at, as_of) {
        Some(age) if (-1..=max_age).contains(&age) => "fresh",
        Some(_) => "stale",
        None => "invalid",
    }
}

fn age_days(value: &str, as_of: &str) -> Option<i64> {
    if !valid_utc(value) || !valid_utc(as_of) {
        return None;
    }
    Some(date_day(as_of)? - date_day(value)?)
}

fn date_day(value: &str) -> Option<i64> {
    Some(epoch_day(
        value[0..4].parse::<i64>().ok()?,
        value[5..7].parse::<i64>().ok()?,
        value[8..10].parse::<i64>().ok()?,
    ))
}

fn epoch_day(mut year: i64, month: i64, day: i64) -> i64 {
    year -= i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn valid_range(value: &str, start: usize, end: usize, min: u8, max: u8) -> bool {
    value[start..end]
        .parse::<u8>()
        .is_ok_and(|part| (min..=max).contains(&part))
}
