use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
use chrono::{DateTime, Utc};
use notify_rust::Notification;
use tracing::error;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use tracing::{info, instrument::WithSubscriber};
use xilem::masonry::vello::wgpu::wgt::error;

use crate::utils::format_as_secs_minutes_and_hours;

pub(crate) async fn move_notif(duration_elapsed: Duration) -> notify_rust::error::Result<()> {
    let body = format!(
        "Move a bit!\n({} elapsed)",
        format_as_secs_minutes_and_hours(duration_elapsed)
    );

    let mut notif = Notification::new();

    notif
        .summary("Time to move!")
        .body(body.as_str())
        .timeout(Duration::from_secs(10));

    #[cfg(not(target_os = "macos"))]
    notif.urgency(notify_rust::Urgency::Critical);

    #[cfg(target_os = "macos")]
    notif.schedule(Utc::now());

    // the handle could be used for actions, but there aren't any at the moment
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let handle = notif.show_async().await?;
    handle.on_close(|reason| info!("closed: {:?}", reason));

    Ok(())
}
