#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // Full-screen quad
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);
    let bottom_left = Vertex { position: [-1.0, -1.0] };
    let top_left = Vertex { position: [-1.0, 1.0] };
    let top_right = Vertex { position: [1.0, 1.0] };
    let bottom_right = Vertex { position: [1.0, -1.0] };
    let shape = vec![bottom_left, top_left, top_right, bottom_right];
    let vertex_buffer = glium::VertexBuffer::immutable(&display, &shape).unwrap();
    let indices = vec![0u16, 1, 2, 2, 3, 0];
    let index_buffer = glium::IndexBuffer::immutable(&display,
                                                     glium::index::PrimitiveType::TrianglesList,
                                                     &indices)
                           .unwrap();
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.416, 0.353, 0.804, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None)
                      .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer,
                    &index_buffer,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
