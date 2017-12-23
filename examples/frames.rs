extern crate hydraulic;

use hydraulic::view::{color, View};
use hydraulic::view::button::Button;
use hydraulic::view::frame::Frame;

fn main() {
    let mut main_view = View::new(
        100.0,
        100.0,
        100,
        100,
        color::white(),
        Box::from(Frame::new()),
    );
    main_view.add_child(View::new(
        600.0,
        600.0,
        10,
        10,
        color::blue(),
        Box::from(Frame::new()),
    ));

    let mut application = hydraulic::Application::new(1024, 768);
    application.run(&mut main_view);
}
