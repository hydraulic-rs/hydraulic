extern crate cgmath;
#[macro_use]
extern crate glium;

pub mod view;

use cgmath::{Matrix4, Vector3};
use glium::{glutin, Surface};
use view::View;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

pub struct Application {
    width: u32,
    height: u32,

    display: Option<glium::Display>,
    program: Option<glium::Program>,

    vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
    indices: Option<glium::index::NoIndices>,
}

impl Application {
    pub fn new(width: u32, height: u32) -> Application {
        Application {
            width: width,
            height: height,

            display: None,
            program: None,

            vertex_buffer: None,
            indices: None,
        }
    }

    pub fn run(&mut self, main_view: &mut View) {
        let mut events_loop = Some(glutin::EventsLoop::new());
        let window = glutin::WindowBuilder::new()
            .with_title("Hydraulic Example: Frames")
            .with_dimensions(self.width, self.height);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        self.display =
            Some(glium::Display::new(window, context, &events_loop.as_ref().unwrap()).unwrap());
        self.display.as_ref().unwrap();

        implement_vertex!(Vertex, position);

        let vertex1 = Vertex {
            position: [0.0, 0.0],
        };
        let vertex2 = Vertex {
            position: [1.0, 0.0],
        };
        let vertex3 = Vertex {
            position: [0.0, 1.0],
        };
        let vertex4 = Vertex {
            position: [1.0, 1.0],
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

            uniform vec4 color;
            out vec4 output_color;

            void main() {
                output_color = color;
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

        let mut closed = false;
        let mut cursor_pos_x: f32 = 0.0;
        let mut cursor_pos_y: f32 = 0.0;

        while !closed {
            let mut target = self.display.as_ref().unwrap().draw();
            target.clear_color(0.05, 0.05, 0.60, 1.0);

            self.render_view_hierarchy(main_view, 0.0, 0.0, &mut target);
            target.finish().unwrap();

            events_loop
                .as_mut()
                .unwrap()
                .poll_events(|event| match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::Closed => closed = true,
                        glutin::WindowEvent::MouseMoved {
                            device_id,
                            position,
                        } => {
                            cursor_pos_x = position.0 as f32;
                            cursor_pos_y = self.height as f32 - position.1 as f32;
                        }
                        glutin::WindowEvent::MouseInput {
                            device_id,
                            state,
                            button,
                        } => if state == glutin::ElementState::Released
                            && button == glutin::MouseButton::Left
                        {
                            println!("LeftMouseClick: {} , {}", cursor_pos_x, cursor_pos_y);
                            self.handle_click_event(main_view, cursor_pos_x, cursor_pos_y);
                        },
                        _ => (),
                    },
                    _ => (),
                });
        }
    }

    fn render_view_hierarchy(&self, view: &View, offset_x: f32, offset_y: f32, target: &mut glium::Frame) {
        let translate_matrix = Matrix4::from_translation(Vector3::new(
            ((view.pos_x + offset_x) / self.width as f32 * 2.0) - 1.0,
            ((view.pos_y + offset_y) / self.height as f32 * 2.0) - 1.0,
            0.0,
        ));

        let scale_matrix = Matrix4::from_nonuniform_scale(
            (view.width as f32 / self.width as f32 * 2.0),
            (view.height as f32 / self.height as f32 * 2.0),
            1.0,
        );

        let final_matrix = translate_matrix * scale_matrix;

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(final_matrix),
            color: Into::<[f32; 4]>::into(view.color),
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
            self.render_view_hierarchy(child, offset_x + view.pos_x, offset_y + view.pos_y, target);
        }
    }

    fn handle_click_event(&self, view: &mut View, pos_x: f32, pos_y: f32) {
        if pos_x >= view.pos_x && pos_x <= view.pos_x + view.width as f32 && pos_y >= view.pos_y
            && pos_y <= view.pos_y + view.height as f32
        {
            view.scene_component.as_mut().mouse_input(
                pos_x as u32,
                pos_y as u32,
                view::MouseButton::Left,
                view::MouseButtonState::Released,
            );
        }

        for child in view.children_mut().iter_mut() {
            self.handle_click_event(child, pos_x, pos_y);
        }
    }
}
