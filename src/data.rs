use std::time::Duration;

use xilem::tokio::runtime::Runtime;

pub(crate) const DEFAULT_DURATION: Duration = Duration::from_mins(15);

#[derive(Debug)]
pub(crate) struct AppData {
    pub(crate) progress: Duration,
    pub(crate) total: Duration,
    pub(crate) paused: bool,
    pub(crate) notif_sent: bool,
    pub(crate) tokio_runtime: Runtime,
    pub(crate) hour_input: String,
    pub(crate) min_input: String,
    pub(crate) sec_input: String,
}

impl AppData {
    pub(crate) fn new(duration: Duration) -> std::io::Result<AppData> {
        Ok(AppData {
            progress: Duration::ZERO,
            total: duration,
            paused: true,
            notif_sent: false,
            tokio_runtime: Runtime::new()?,
            hour_input: String::new(),
            min_input: String::new(),
            sec_input: String::new(),
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
}
