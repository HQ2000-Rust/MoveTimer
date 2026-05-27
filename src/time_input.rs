use xilem::{
    WidgetView,
    view::{GridExt, GridParams, grid, text_button, text_input},
};

use crate::{
    data::{AppData, DEFAULT_DURATION},
    utils::hours_mins_secs,
};

pub(crate) fn time_input(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    grid(
        (
            grid(
                (
                    //TODO: labels!!
                    text_input(
                        data.sec_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.sec_input = keep_numbers(new_content);

                            //0 if it's an empty string or overflow
                            data.sec_parsed = data.sec_input.as_str().parse().unwrap_or(0);
                        },
                    )
                    .grid_pos(0, 0),
                    text_input(
                        data.min_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.min_input = keep_numbers(new_content);

                            //0 if it's an empty string or overflow
                            data.min_parsed = data.min_input.as_str().parse().unwrap_or(0);
                        },
                    )
                    .grid_pos(0, 1),
                    text_input(
                        data.hour_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            data.hour_input = keep_numbers(new_content);

                            //0 if it's an empty string or overflow
                            data.hour_parsed = data.hour_input.as_str().parse().unwrap_or(0);
                        },
                    )
                    .grid_pos(0, 2),
                ),
                1,
                3,
            )
            .grid_item(GridParams::new(0, 0, 1, 1)),
            //different
            grid(
                (
                    text_button("Reset", |data: &mut AppData| {
                       // let (default_hours, default_mins, default_secs) =
                       //     hours_mins_secs(DEFAULT_DURATION);
                        // data.sec_input = default_secs.to_string();
                       // data.min_input = default_mins.to_string();
                      //  data.hour_input = default_hours.to_string();
                      // 
                      data.set_new_duration(DEFAULT_DURATION);
                    })
                    .disabled(
                        DEFAULT_DURATION
                        
                            == data.total,
                    )
                    .grid_item(GridParams::new(0, 0, 1, 1)),
                    text_button("Apply", |data: &mut AppData| {
                        data.set_new_duration(data.input_duration());
                    })
                    .disabled(data.total == data.input_duration())
                    .grid_item(GridParams::new(0, 1, 1, 1)),
                ),
                1,
                2,
            )
            .grid_item(GridParams::new(1, 0, 1, 1)),
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
