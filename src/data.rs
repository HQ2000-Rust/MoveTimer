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
}
