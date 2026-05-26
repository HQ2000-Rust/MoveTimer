use std::{fmt::format, sync::Arc, time::Duration};

use crate::data::AppData;
use xilem::{
    TextAlign, WidgetView,
    masonry::properties::types::Length,
    view::{FlexExt, FlexSpacer, flex_col, flex_row, label, progress_bar},
};

pub(crate) fn time_view(total: Duration, progress: Duration) -> impl WidgetView<AppData> + use<> {
    let relative_progress = total.as_millis() as f64 / progress.as_millis() as f64;

    //FIXME
    assert!(relative_progress < 1.);

    flex_col((
        progress_bar(Some(relative_progress)),
        flex_row((
            label(format_as_secs_minutes_and_hours(progress))
                .text_alignment(TextAlign::Start)
                .text_size(25.)
                .flex(0.5),
            label(format_as_secs_minutes_and_hours(total))
                .text_alignment(TextAlign::End)
                .text_size(25.)
                .flex(0.5),
        )),
    ))
}

fn format_as_secs_minutes_and_hours(duration: Duration) -> impl Into<Arc<str>> {
    let total_secs = duration.as_secs();

    let secs = total_secs % 60;

    let remaining_secs = total_secs - secs;

    let mins = (remaining_secs / 60) % 60;

    let remaining_mins = (remaining_secs / 60) - mins;

    let hours = (remaining_mins / 60) as u64;

    //not the final form
    match (secs==0, mins==0, hours==0) {
        //0s fallback
        (_,false, false) => format!("{}s", secs),
        (_, true, false) => format!("{}s {}m", secs, mins),
        (_, _, true) => format!("{}s {}m {}h", secs, mins, hours),
    }
}
