use std::{sync::Arc, time::Duration};

use crate::data::AppData;
use xilem::{
    TextAlign, WidgetView,
    masonry::properties::types::Length,
    palette::css::WHITE,
    style::Style,
    view::{
        Axis, FlexExt, FlexSpacer, ZStackExt, button, flex, flex_col, flex_row, label,
        progress_bar, text_button,
    },
};

pub(crate) fn time_view(total: Duration, progress: Duration) -> impl WidgetView<AppData> + use<> {
    let relative_progress = total.as_millis() as f64 / progress.as_millis() as f64;

    //FIXME
    assert!(relative_progress < 1.);


    
    flex(
        Axis::Vertical,
        (
            progress_bar(Some(relative_progress)),
            
            flex_row((
                label(format_as_secs_minutes_and_hours(progress))
                    .text_alignment(TextAlign::Start)
                    .text_size(25.),
                   



                FlexSpacer::Flex(1.),
                label(
                    "0s"//format_as_secs_minutes_and_hours(total)
                )
                .text_alignment(TextAlign::End)
                .text_size(25.).flex(0.5),
                

                //.flex(0.5),
            ))
            .main_axis_alignment(xilem::view::MainAxisAlignment::Center)
            .cross_axis_alignment(xilem::view::CrossAxisAlignment::Center)
            .must_fill_major_axis(true)
            //.flex(2.0),
        ),
    )
}

fn format_as_secs_minutes_and_hours(duration: Duration) -> impl Into<Arc<str>> {
    let total_secs = duration.as_secs();

    let secs = total_secs % 60;

    let remaining_secs = total_secs - secs;

    let mins = (remaining_secs / 60) % 60;

    let remaining_mins = (remaining_secs / 60) - mins;

    let hours = (remaining_mins / 60) as u64;

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
        format!("{} {} {}", fmt(hours, "h"), fmt(mins, "m"), fmt(secs, "s"))
    }
}
