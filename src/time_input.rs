use xilem::{
    WidgetView,
    view::{GridExt, grid, text_button, text_input},
};

use crate::{
    data::{AppData, DEFAULT_DURATION},
    utils::{duration_from_secs_mins_hours, hours_mins_secs},
};

pub(crate) fn time_input(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    grid(
        (
            grid(
                (
                    text_input(
                        data.sec_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.sec_input = keep_numbers(new_content);
                        },
                    )
                    .grid_pos(0, 0),
                    text_input(
                        data.min_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.min_input = keep_numbers(new_content);
                        },
                    )
                    .grid_pos(0, 1),
                    text_input(
                        data.hour_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.hour_input = keep_numbers(new_content);
                        },
                    )
                    .grid_pos(0, 2),
                ),
                1,
                3,
            )
            .grid_pos(0, 0),
            grid(
                (
                    text_button("Reset", |data: &mut AppData| {
                        let (default_hours, default_mins, default_secs) =
                            hours_mins_secs(DEFAULT_DURATION);
                        data.sec_input = default_secs.to_string();
                        data.min_input = default_mins.to_string();
                        data.hour_input = default_hours.to_string();
                    })
                    .disabled(
                        DEFAULT_DURATION
                        //will need to refactor this...
                            == duration_from_secs_mins_hours(
                                //just to be safe, maybe remove later (TODO: e. g. a u64 field `input_secs`)
                                keep_numbers(data.sec_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                                keep_numbers(data.min_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                                keep_numbers(data.hour_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                            ),
                    )
                    .grid_pos(0, 0),
                    text_button("Apply", |data: &mut AppData| {
                        let new_duration = duration_from_secs_mins_hours(
                            //just to be safe, maybe remove later (TODO: e. g. a u64 field `input_secs`)
                            keep_numbers(data.sec_input.clone())
                                .as_str()
                                .parse()
                                .unwrap_or(0),
                            keep_numbers(data.min_input.clone())
                                .as_str()
                                .parse()
                                .unwrap_or(0),
                            keep_numbers(data.hour_input.clone())
                                .as_str()
                                .parse()
                                .unwrap_or(0),
                        );

                        data.set_new_duration(new_duration);
                    })
                    .disabled(
                        //well...
                        data.total
                            == duration_from_secs_mins_hours(
                                //just to be safe, maybe remove later (TODO: e. g. a u64 field `input_secs`)
                                keep_numbers(data.sec_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                                keep_numbers(data.min_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                                keep_numbers(data.hour_input.clone())
                                    .as_str()
                                    .parse()
                                    .unwrap_or(0),
                            ),
                    ),
                ),
                1,
                2,
            ),
        ),
        2,
        1,
    )
}

//extremely efficient
fn keep_numbers(input: String) -> String {
    let mut output = String::new();

    for character in input.chars() {
        if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&character) {
            output.push(character);
        }
    }

    output
}
