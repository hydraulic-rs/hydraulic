pub struct View {
    pub pos_x: f32,
    pub pos_y: f32,
    children: Vec<View>,
}

impl View {
    pub fn new(pos_x: f32, pos_y: f32) -> View {
        View {
            pos_x: pos_x,
            pos_y: pos_y,
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
        let view = View::new(50.0, 40.0);

        assert_eq!(view.pos_x, 50.0);
        assert_eq!(view.pos_y, 40.0);
    }

    #[test]
    fn nested_views() {
        let mut root = View::new(0.0, 0.0);
        root.add_child(View::new(10.0, 10.0));
        root.add_child(View::new(40.0, 40.0));

        assert_eq!(2, root.children().len());
        assert_eq!(40.0, root.children()[1].pos_x);
    }
}
