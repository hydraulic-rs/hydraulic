extern crate cgmath;

use cgmath::Vector4;

pub mod color;
pub mod button;
pub mod frame;

pub struct View {
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: u32,
    pub height: u32,
    pub color: Vector4<f32>,
    pub scene_component: Box<SceneComponent>,
    children: Vec<View>,
}

impl View {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        width: u32,
        height: u32,
        color: Vector4<f32>,
        scene_component: Box<SceneComponent>,
    ) -> View {
        View {
            pos_x: pos_x,
            pos_y: pos_y,
            width: width,
            height: height,
            color: color,
            scene_component: scene_component,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, view: View) {
        self.children.push(view);
    }

    pub fn children(&mut self) -> &mut Vec<View> {
        &mut self.children
    }
}

pub enum MouseButton {
    Left,
    Right,
    Center,
}

pub enum MouseButtonState {
    Released,
}

pub trait SceneComponent {
    fn mouse_input(
        &mut self,
        mouse_x: u32,
        mouse_y: u32,
        mouse_button: MouseButton,
        element_state: MouseButtonState,
    );
}

#[cfg(test)]
mod tests {
    use view::View;
    use view::color;
    use view::frame::Frame;

    #[test]
    fn new_view() {
        let view = View::new(50.0, 40.0, 100, 100, color::white(), Box::new(Frame::new()));

        assert_eq!(view.pos_x, 50.0);
        assert_eq!(view.pos_y, 40.0);
        assert_eq!(view.width, 100);
        assert_eq!(view.height, 100);
    }

    #[test]
    fn nested_views() {
        let mut root = View::new(0.0, 0.0, 10, 10, color::white(), Box::new(Frame::new()));
        root.add_child(View::new(
            10.0,
            10.0,
            10,
            10,
            color::white(),
            Box::new(Frame::new()),
        ));
        root.add_child(View::new(
            40.0,
            40.0,
            10,
            10,
            color::white(),
            Box::new(Frame::new()),
        ));

        assert_eq!(2, root.children().len());
        assert_eq!(40.0, root.children()[1].pos_x);
    }
}
