pub struct View {
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: u32,
    pub height: u32,
    children: Vec<View>,
}

impl View {
    pub fn new(pos_x: f32, pos_y: f32, width: u32, height: u32) -> View {
        View {
            pos_x: pos_x,
            pos_y: pos_y,
            width: width,
            height: height,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, view: View) {
        self.children.push(view);
    }

    pub fn children(&self) -> &Vec<View> {
        &self.children
    }
}


#[cfg(test)]
mod tests {
    use view::View;

    #[test]
    fn new_view() {
        let view = View::new(50.0, 40.0, 100, 100);

        assert_eq!(view.pos_x, 50.0);
        assert_eq!(view.pos_y, 40.0);
        assert_eq!(view.width, 100);
        assert_eq!(view.height, 100);
    }

    #[test]
    fn nested_views() {
        let mut root = View::new(0.0, 0.0, 10, 10);
        root.add_child(View::new(10.0, 10.0, 10, 10));
        root.add_child(View::new(40.0, 40.0, 10, 10));

        assert_eq!(2, root.children().len());
        assert_eq!(40.0, root.children()[1].pos_x);
    }
}
