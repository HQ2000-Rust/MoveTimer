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
pub(crate) mod notif;
pub(crate) mod time_view;
pub(crate) mod utils;

use data::AppData;

use crate::time_view::time_view;

const TICK: Duration = Duration::from_millis(100);

#[derive(Debug)]
enum Message {
    Tick,
}

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
                    label(match (data.paused, data.total == data.progress) {
                        (true, false) => "Resume",
                        (_, true) => "Reset",
                        (false, false) => "Pause",
                    }),
                    |data_: &mut AppData| {
                        match (data_.paused, data_.total == data_.progress) {
                            (false, true) => {
                                data_.paused = true;
                                data_.progress = Duration::ZERO;
                                data_.notif_sent = false;
                            }
                            (_, false) => {
                                data_.paused = !data_.paused;
                            }
                            //possible edge case, handled explicitly (future changes!!)
                            // (maybe just remove this?)
                            (true, true) => {
                                data_.paused = true;
                                data_.progress = Duration::ZERO;
                                data_.notif_sent = false;
                            }
                        };
                    },
                )
                .grid_item(GridParams::new(0, 7, 3, 1)),
            ),
            3,
            8,
        ),
        // FIXME: always ticking, even when finished/pausing
        // clean solution?
        data.paused.not().then(|| {
            task(
                |proxy| async move {
                    loop {
                        tokio::time::sleep(TICK).await;
                        if let Ok(()) = proxy.message(Message::Tick) {
                        } else {
                            break;
                        }
                    }
                },
                |data: &mut AppData, _msg| {
                    if !data.paused {
                        data.advance_timer_by(TICK);
                        if data.total == data.progress && !data.notif_sent {
                            //TODO: resend if an error occurs?

                            let _result = data.tokio_runtime.spawn(notif::move_notif(data.total));
                            data.notif_sent = true;
                        }
                    }
                },
            )
        }),
    )
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        //TODO: gracefully handle the error(s)
        AppData::new(Duration::from_secs(2)).unwrap(),
        app_logic,
        WindowOptions::new("MoveTimer"),
    );

    app.run_in(EventLoop::with_user_event())
}
