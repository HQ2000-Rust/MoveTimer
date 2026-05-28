use std::{fmt::format, time::Duration};

use xilem::tokio::runtime::Runtime;

use crate::{
    notif::NotifSettings,
    utils::{duration_from_secs_mins_hours, hours_mins_secs},
};

pub(crate) const DEFAULT_DURATION: Duration = Duration::from_mins(15);

#[derive(Debug)]
pub(crate) struct AppData {
    pub(crate) progress: Duration,
    pub(crate) total: Duration,
    pub(crate) paused: bool,
    pub(crate) notif_sent: bool,
    pub(crate) tokio_runtime: Runtime,
    pub(crate) hour_input: String,
    pub(crate) hour_parsed: u64,
    pub(crate) min_input: String,
    pub(crate) min_parsed: u64,
    pub(crate) sec_input: String,
    pub(crate) sec_parsed: u64,
    pub(crate) settings: NotifSettings,
    pub(crate) input_settings: NotifSettings,
}

impl AppData {
    pub(crate) fn new(duration: Duration) -> std::io::Result<AppData> {
        let (default_hour, default_min, default_sec) = hours_mins_secs(duration);

        Ok(AppData {
            progress: Duration::ZERO,
            total: duration,
            paused: true,
            notif_sent: false,
            tokio_runtime: Runtime::new()?,
            hour_input: format!("{}h", default_hour),
            hour_parsed: default_hour,
            min_input: format!("{}m", default_min),
            min_parsed: default_min as u64,
            sec_input: format!("{}s", default_sec),
            sec_parsed: default_sec as u64,
            settings: NotifSettings::default(),
            input_settings: NotifSettings::default(),
        })
    }

    pub(crate) fn advance_timer_by(&mut self, duration: Duration) {
        let new_progress = self.progress + duration;

        self.progress = if new_progress >= self.total {
            self.total
        } else {
            new_progress
        };
    }

    pub(crate) fn set_new_duration(&mut self, new_duration: Duration) {
        self.total = new_duration;

        if self.progress > new_duration {
            self.progress = new_duration;
        }
    }

    pub(crate) fn input_duration(&self) -> Duration {
        duration_from_secs_mins_hours(self.sec_parsed, self.min_parsed, self.hour_parsed)
    }
}
