use std::ops::Not;
use std::time::Duration;

use xilem::core::fork;


use xilem::view::{
    GridExt, GridParams, button, grid, label, task,
};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, FontWeight, TextAlign, WidgetView, WindowOptions, Xilem, tokio};

pub(crate) mod data;
pub(crate) mod notif;
pub(crate) mod time_input;
pub(crate) mod time_view;
pub(crate) mod utils;

use data::AppData;

use crate::data::DEFAULT_DURATION;
use crate::time_input::time_input;
use crate::time_view::time_view;
use crate::utils::format_as_secs_minutes_and_hours;

const TICK: Duration = Duration::from_millis(100);

#[derive(Debug)]
enum Message {
    Tick,
}

const BUTTON_TEXT_SIZE: f32 = 15.;

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    fork(
        grid(
            (
                label("MoveTimer")
                    .text_alignment(TextAlign::Center)
                    .text_size(60.)
                    .weight(FontWeight::parse("bold").unwrap())
                    .grid_item(GridParams::new(0, 0, 3, 1)),
                time_view(data.total, data.progress).grid_item(GridParams::new(0, 1, 3, 1)),
                button(
                    label(match (data.paused, data.total == data.progress) {
                        (true, false) => "Resume",
                        (_, true) => "Reset",
                        (false, false) => "Pause",
                    })
                    .text_size(25.),
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
                .grid_item(GridParams::new(0, 4, 3, 1)),
                time_input(data).grid_pos(0, 3),
                button(
                    label("Set elapsed time to 0")
                       // .text_alignment(TextAlign::Justify)
                        .text_size(BUTTON_TEXT_SIZE),
                    |data: &mut AppData| {
                        data.progress = Duration::ZERO;
                    },
                )
                .grid_pos(1, 3),
                button(
                    label(format!(
                        "Reset to default ({})",
                        format_as_secs_minutes_and_hours(DEFAULT_DURATION)
                    ))
                    //.text_alignment(TextAlign::Justify),
                    .text_size(BUTTON_TEXT_SIZE),
                    |data: &mut AppData| {
                        data.set_new_duration(DEFAULT_DURATION);
                    },
                )
                .disabled(DEFAULT_DURATION == data.total)
                .grid_pos(2, 3), //timeout only on xdg desktops
                                 /*slider(
                                     0.,
                                     MAX_SECS_NOTIF_DURATION as f64,
                                     data.input_settings.duration.as_secs() as f64,
                                     |data: &mut AppData, new_val| {
                                         // Duration::from_secs_f64 sadly not stable yet
                                         data.input_settings.duration = Duration::from_secs(new_val as u64);
                                     },
                                 )
                                 .grid_item(GridParams::new(2, 5, 2, 1)),
                                 label(data.input_settings.duration.as_secs().to_string()).grid_pos(2, 6),
                                 text_button("apply notif settings", |data: &mut AppData| {
                                     data.settings = data.input_settings.clone();
                                 })
                                 .disabled(data.settings == data.input_settings),*/
            ),
            3,
            5,
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

                            let _result = data
                                .tokio_runtime
                                .spawn(notif::move_notif(data.total, data.settings.clone()));
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
        AppData::new(DEFAULT_DURATION).unwrap(),
        app_logic,
        WindowOptions::new("MoveTimer"),
    );

    app.run_in(EventLoop::with_user_event())
}
