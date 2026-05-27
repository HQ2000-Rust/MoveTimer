use std::time::{Duration, Instant};

#[cfg(target_os = "macos")]
use chrono::{DateTime, Utc};
use notify_rust::Notification;
use tracing::error;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use tracing::{info, instrument::WithSubscriber};
use xilem::masonry::vello::wgpu::wgt::error;

use crate::utils::format_as_secs_minutes_and_hours;

//TODO: proper gating according to the docs
pub(crate) async fn move_notif(
    duration_elapsed: Duration,
    settings: NotifSettings,
) -> notify_rust::error::Result<()> {
    let body = format!(
        "Move a bit!\n({} elapsed)",
        format_as_secs_minutes_and_hours(duration_elapsed)
    );

    let mut notif = Notification::new();

    notif.summary("Time to move!").body(body.as_str());

    #[cfg(not(target_os = "macos"))]
    {
        notif.timeout(settings.duration);

        if let Some(name) = settings.sound_name {
            notif.sound_name(name.as_str());
        }

        notif.urgency(notify_rust::Urgency::Critical);
    }

    #[cfg(target_os = "macos")]
    notif.schedule(Utc::now());

    // the handle could be used for actions, but there aren't any at the moment
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let handle = notif.show_async().await?;
    handle.on_close(|reason| info!("closed: {:?}", reason));

    Ok(())
}

//non-persistent notif settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NotifSettings {
    pub(crate) duration: Duration,

    pub(crate) sound_name: Option<String>,
}

impl Default for NotifSettings {
    fn default() -> Self {
        NotifSettings {
            duration: Duration::from_mins(1),
            sound_name: None,
        }
    }
}
