use xilem::{
    TextAlign, WidgetView,
    view::{
        FlexExt, GridExt, GridParams, MainAxisAlignment, button, flex, flex_col, flex_row, grid, label, text_button, text_input, zstack
    },
};

use crate::{
    BUTTON_TEXT_SIZE, data::{AppData, DEFAULT_DURATION}, utils::{format_as_secs_minutes_and_hours, hours_mins_secs}
};

const S_M_H_TEXT_SIZE: f32 = 20.;

//TODO: decide how to handle the Duration::ZERO (user input) case, atm possibly inconsistent output: "resume" + "reset"
pub(crate) fn time_input(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
    grid(
        (
            //flex_row((
            grid(
                (
                    text_input(
                        data.hour_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            let mut numbers = keep_digits(new_content);

                            //0 if it's an empty string or overflow
                            data.hour_parsed = numbers.as_str().parse().unwrap_or(0);

                            if !numbers.is_empty() {
                                numbers.push('h');
                            }
                            data.hour_input = numbers;
                        },
                    )
                    .grid_item(GridParams::new(0, 0, 1, 1)),
                    text_input(
                        data.min_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            let mut numbers = keep_digits(new_content);

                            //0 if it's an empty string or overflow
                            data.min_parsed = numbers.as_str().parse().unwrap_or(0);

                            if !numbers.is_empty() {
                                numbers.push('m');
                            }
                            data.min_input = numbers;
                        },
                    )
                    .grid_item(GridParams::new(0, 1, 1, 1)),
                    text_input(
                        data.sec_input.clone(),
                        |data: &mut AppData, new_content: String| {
                            let mut numbers = keep_digits(new_content);

                            //0 if it's an empty string or overflow
                            data.sec_parsed = numbers.as_str().parse().unwrap_or(0);

                            if !numbers.is_empty() {
                                numbers.push('s');
                            }
                            data.sec_input = numbers;
                        },
                    )
                    .grid_item(GridParams::new(0, 2, 1, 1)),
                ),
                1,
                3,
            )
            /* .flex(3.),
            grid(
                (
                    label("s")
                        .text_size(S_M_H_TEXT_SIZE)
                        //.text_alignment(TextAlign::End)
                        .grid_pos(0, 0),
                    label("m").text_size(S_M_H_TEXT_SIZE).grid_pos(0, 1),
                    label("h").text_size(S_M_H_TEXT_SIZE).grid_pos(0, 2),
                ),
                1,
                3,
            )
            */
            //.flex(0.5)
            //))
            // .main_axis_alignment(MainAxisAlignment::SpaceAround)
            //.must_fill_major_axis(true)
            .grid_item(GridParams::new(0, 0, 1, 1)),
            button(label("Apply").text_size(BUTTON_TEXT_SIZE), |data: &mut AppData| {
                data.set_new_duration(data.input_duration());
            })
            .disabled(data.total == data.input_duration())
            .grid_item(GridParams::new(0, 1, 1, 1)),
        ),
        1,
        2,
    )
}

//extremely efficient
fn keep_digits(input: String) -> String {
    let mut output = String::new();

    for character in input.chars() {
        if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&character) {
            output.push(character);
        }
    }

    output
}
