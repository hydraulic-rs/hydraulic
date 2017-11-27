#[macro_use]
extern crate glium;

pub mod view;

use glium::{glutin, Surface};
use view::View;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

pub struct Application {
    width: u32,
    height: u32,
    main_view: View,

    events_loop: Option<glutin::EventsLoop>,
    display: Option<glium::Display>,
    program: Option<glium::Program>,

    vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
    indices: Option<glium::index::NoIndices>,
}

impl Application {
    pub fn new(width: u32, height: u32, main_view: View) -> Application {
        Application {
            width: width,
            height: height,
            main_view: main_view,

            events_loop: None,
            display: None,
            program: None,

            vertex_buffer: None,
            indices: None,
        }
    }

    pub fn init(&mut self) {
        self.events_loop = Some(glutin::EventsLoop::new());
        let window = glutin::WindowBuilder::new()
            .with_title("Hydraulic Example: Frames")
            .with_dimensions(self.width, self.height);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        self.display = Some(glium::Display::new(window, context, &self.events_loop.as_ref().unwrap()).unwrap());
        self.display.as_ref().unwrap();

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex {
            position: [0.0, 0.0],
        };
        let vertex2 = Vertex {
            position: [0.0, 0.5],
        };
        let vertex3 = Vertex {
            position: [0.5, 0.0],
        };
        let vertex4 = Vertex {
            position: [0.5, 0.5],
        };
        let shape = vec![vertex1, vertex2, vertex3, vertex4];

        self.vertex_buffer =
            Some(glium::VertexBuffer::new(self.display.as_ref().unwrap(), &shape).unwrap());
        self.indices = Some(glium::index::NoIndices(
            glium::index::PrimitiveType::TriangleStrip,
        ));

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 matrix;

            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.0, 0.0, 0.0, 1.0);
            }
        "#;

        self.program = Some(
            glium::Program::from_source(
                self.display.as_ref().unwrap(),
                vertex_shader_src,
                fragment_shader_src,
                None,
            ).unwrap(),
        );
    }

    pub fn run(&mut self) {
        let mut closed = false;
        while !closed {
            let mut target = self.display.as_ref().unwrap().draw();
            target.clear_color(0.05, 0.05, 0.05, 1.0);

            self.render_view_hierarchy(&self.main_view, &mut target);
            target.finish().unwrap();

            self.events_loop
                .as_mut()
                .unwrap()
                .poll_events(|event| match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => closed = true,
                        _ => (),
                    },
                    _ => (),
                });
        }
    }

    fn render_view_hierarchy(&self, view: &View, target: &mut glium::Frame) {
        let mx = (view.pos_x / self.width as f32) * 2.0 - 1.0;
        let my = (view.pos_y / self.height as f32) * 2.0 - 1.0;

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [mx, my, 0.0, 1.0f32],
            ]
        };

        target
            .draw(
                self.vertex_buffer.as_ref().unwrap(),
                self.indices.as_ref().unwrap(),
                &self.program.as_ref().unwrap(),
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        for child in view.children().iter() {
            self.render_view_hierarchy(child, target);
        }
    }
}
