extern crate hydraulic;

use hydraulic::view::View;

fn main() {
    let mut main_view = View::new(0.0, 0.0);
    main_view.add_child(View::new(100.0, 300.0));
    main_view.add_child(View::new(300.0, 500.0));
    
    let mut application = hydraulic::Application::new(1024, 768, main_view);
    application.init();
    application.run();
}
