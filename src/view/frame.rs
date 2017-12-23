use view::*;

pub struct Frame {}

impl Frame {
    pub fn new() -> Frame {
        Frame {}
    }
}

impl SceneComponent for Frame {
    fn mouse_input(
        &mut self,
        mouse_x: u32,
        mouse_y: u32,
        mouse_button: MouseButton,
        element_state: MouseButtonState,
    ) {
    }
}
