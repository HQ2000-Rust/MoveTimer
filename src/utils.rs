use std::time::Duration;
use std::sync::Arc;

pub(crate) fn format_as_secs_minutes_and_hours(duration: Duration) -> String {
    let total_secs = duration.as_secs();

    let secs = total_secs % 60;

    let remaining_secs = total_secs - secs;

    let mins = (remaining_secs / 60) % 60;

    let remaining_mins = (remaining_secs / 60) - mins;

    let hours = (remaining_mins / 60) as u64;

    let fmt = |n: u64, suff: &str| {
        if n != 0 {
            format!("{}{}", n.to_string(), suff)
        } else {
            String::new()
        }
    };
    if secs == 0 && hours == 0 && mins == 0 {
        "0s".to_string()
    } else {
        format!("{} {} {}", fmt(hours, "h"), fmt(mins, "m"), fmt(secs, "s"))
    }
}
