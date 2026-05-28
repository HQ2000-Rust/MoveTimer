use std::time::Duration;

pub(crate) fn format_as_secs_minutes_and_hours(duration: Duration) -> String {
    let (hours, mins, secs) = hours_mins_secs(duration);

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
        //cheating...
        format!(
            "{}{}{}",
            fmt(hours, "h "),
            fmt(mins as u64, "m "),
            fmt(secs as u64, "s")
        )
    }
}

pub(crate) fn hours_mins_secs(duration: Duration) -> (u64, u8, u8) {
    let total_secs = duration.as_secs();

    let secs = total_secs % 60;

    let remaining_secs = total_secs - secs;

    let mins = (remaining_secs / 60) % 60;

    let remaining_mins = (remaining_secs / 60) - mins;

    let hours = remaining_mins / 60;

    (hours, mins as u8, secs as u8)
}

pub(crate) fn duration_from_secs_mins_hours(secs: u64, mins: u64, hours: u64) -> Duration {
    Duration::from_secs(secs) + Duration::from_mins(mins) + Duration::from_hours(hours)
}
