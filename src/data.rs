use std::time::Duration;

use xilem::tokio::runtime::Runtime;

#[derive(Debug)]
pub(crate) struct AppData {
    pub(crate) progress: Duration,
    pub(crate) total: Duration,
    pub(crate) paused: bool,
    pub(crate) notif_sent: bool,
    pub(crate) tokio_runtime: Runtime,
}

impl AppData {
    pub(crate) fn new(duration: Duration) -> std::io::Result<AppData> {
        Ok(AppData {
            progress: Duration::ZERO,
            total: duration,
            paused: true,
            notif_sent: false,
            tokio_runtime: Runtime::new()?
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
}
