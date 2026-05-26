use std::time::Duration;

use xilem::palette::css::{BLACK, WHITE};
use xilem::style::{Background, Padding, Style};
use xilem::view::{
    Axis, FlexSpacer, GridExt, GridParams, Label, button, flex, flex_col, grid, label, text_button,
};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, TextAlign, WidgetView, WindowOptions, Xilem};

pub(crate) mod data;
pub(crate) mod time_view;

use data::AppData;

use crate::time_view::time_view;

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
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
