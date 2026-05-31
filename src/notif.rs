use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
use chrono::{DateTime, Utc};
use notify_rust::Notification;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use tracing::info;

use crate::utils::format_as_secs_minutes_and_hours;

//TODO: proper gating according to the docs
pub(crate) async fn move_notif(duration_elapsed: Duration) -> notify_rust::error::Result<()> {
    let body = format!(
        "Move a bit!\n({} elapsed)",
        format_as_secs_minutes_and_hours(duration_elapsed)
    );

    let mut notif = Notification::new();

    notif.summary("Time to move!").body(body.as_str());

    // XDG settings
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        notif.timeout(Duration::from_secs(60));

        notif.sound_name("ping");

        notif.urgency(notify_rust::Urgency::Critical);
    }

    #[cfg(target_os = "macos")]
    notif.schedule(Utc::now());

    // the handle could be used for actions, but there aren't any at the moment
    //#[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let handle = notif.show_async().await?;
    handle.on_close(|reason| info!("closed: {:?}", reason));

    Ok(())
}
