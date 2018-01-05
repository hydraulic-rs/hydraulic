extern crate hydraulic;

use hydraulic::view::{color, View};
use hydraulic::view::button::Button;
use hydraulic::view::frame::Frame;

fn main() {
    let mut main_view = View::new(
        100.0,
        100.0,
        300,
        300,
        color::white(),
        Box::from(Frame::new()),
    );
    main_view.add_child(View::new(
        10.0,
        10.0,
        130,
        130,
        color::red(),
        Box::from(Frame::new()),
    ));
    main_view.add_child(View::new(
        160.0,
        10.0,
        130,
        130,
        color::green(),
        Box::from(Frame::new()),
    ));
    main_view.add_child(View::new(
        10.0,
        150.0,
        280,
        130,
        color::black(),
        Box::from(Frame::new()),
    ));

    let mut application = hydraulic::Application::new(1024, 768);
    application.run(&mut main_view);
}
