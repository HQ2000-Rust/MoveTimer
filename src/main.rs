use xilem::winit::error::EventLoopError;
use xilem::view::{Axis, Label, button, flex, label, text_button};
use xilem::{EventLoop, WindowOptions, WidgetView, Xilem};

pub(crate) mod data;
pub(crate) mod time_view;

use data::AppData;


fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> + use<> {
let label=label("Hello").text_size(100.);
    
    button(label, |_: &mut AppData| {
        
    })
}

fn main() -> Result<(), EventLoopError> {
    let app=Xilem::new_simple(AppData::new(), app_logic, WindowOptions::new("MoveTimer"));

    app.run_in(EventLoop::with_user_event())
}
