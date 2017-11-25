extern crate hydraulic;

fn main() {
    let application = hydraulic::Application::new(1024, 768);
    application.run();
}
