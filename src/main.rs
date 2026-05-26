use std::ops::Not;
use std::time::Duration;

use xilem::core::fork;
use xilem::palette::css::{BLACK, WHITE};
use xilem::style::{Background, Padding, Style};
use xilem::view::{
    FlexSpacer, GridExt, GridParams, Label, Task, button, flex, flex_col, grid, label, task,
    text_button,
};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, TextAlign, WidgetView, WindowOptions, Xilem, tokio};

pub(crate) mod data;
pub(crate) mod time_view;

use data::AppData;

use crate::time_view::time_view;

const TICK_RESOLUTION: Duration = Duration::from_millis(100);

#[derive(Debug)]
struct Tick;

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    fork(
        grid(
            (
                label("MoveTimer")
                    .text_alignment(TextAlign::Center)
                    .text_size(60.)
                    .grid_item(GridParams::new(0, 0, 3, 1)),
                time_view(data.total, data.progress).grid_item(GridParams::new(0, 1, 3, 1)),
                button(
                    label(if data.paused { "Resume" } else { "Pause" }),
                    |data_: &mut AppData| {
                        data_.paused = !data_.paused;
                    },
                )
                .grid_item(GridParams::new(0, 7, 3, 1)),
            ),
            3,
            8,
        ),
        data.paused.not().then(|| {
        task(
            |proxy| async move {
                loop {
                    tokio::time::sleep(TICK_RESOLUTION).await;
                    if let Ok(()) = proxy.message(Tick) {
                    } else {
                        break;
                    }
                }
            },
            |data: &mut AppData, _msg| {
                if !data.paused {
                    data.advance_timer_by(TICK_RESOLUTION);
                }
            },
        )
        })
    )
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppData::new(Duration::from_mins(15)),
        app_logic,
        WindowOptions::new("MoveTimer"),
    );

    app.run_in(EventLoop::with_user_event())
}
