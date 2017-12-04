extern crate hydraulic;

use hydraulic::view::{color, View};

fn main() {
    let mut main_view = View::new(100.0, 100.0, 100, 100, color::white());
    main_view.add_child(View::new(600.0, 600.0, 10, 10, color::blue()));

    let mut application = hydraulic::Application::new(1024, 768, main_view);
    application.run();
}
