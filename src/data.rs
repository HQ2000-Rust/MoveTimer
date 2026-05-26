use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct AppData {
    pub(crate) progress: Duration,
    pub(crate) total: Duration,
    pub(crate) paused: bool,
}

impl AppData {
    pub(crate) fn new(duration: Duration) -> AppData {
        AppData {
            progress: Duration::ZERO,
            total: duration,
            paused: true,
        }
    }

    pub(crate) fn advance_timer_by(&mut self, duration: Duration) {
        let new_progress = self.progress + duration;

        self.progress = if new_progress > self.total {
            self.total
        } else {
            new_progress
        }
    }
}
