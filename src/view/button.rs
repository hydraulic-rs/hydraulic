use view::*;

pub struct Button {}

impl Button {
    pub fn new() -> Button {
        Button {}
    }

    pub fn trigger(&mut self) {
        println!("Button was triggerd");
    }
}

impl SceneComponent for Button {
    fn mouse_input(
        &mut self,
        mouse_x: u32,
        mouse_y: u32,
        mouse_button: MouseButton,
        element_state: MouseButtonState,
    ) {

    }
}
