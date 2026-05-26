use std::time::Duration;

use xilem::view::{Axis, FlexSpacer, Label, button, flex, flex_col, label, text_button};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

pub(crate) mod data;
pub(crate) mod time_view;

use data::AppData;

use crate::time_view::time_view;

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    flex_col((FlexSpacer::Flex(0.05), time_view(data.progress, data.total)))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppData::new(Duration::from_mins(15)),
        app_logic,
        WindowOptions::new("MoveTimer"),
    );

    app.run_in(EventLoop::with_user_event())
}
