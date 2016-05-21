#[macro_use]
extern crate glium;
extern crate notify;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use notify::{RecommendedWatcher, Watcher};
use std::sync::mpsc::{channel, TryRecvError};

fn string_from_file(mut file: &std::fs::File) -> String {
    let mut src = String::new();
    file.read_to_string(&mut src).unwrap();
    src
}

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

    // Read fragment shader from file
    let fragment_shader_path = Path::new("shader.frag");
    let fragment_shader_path_display = fragment_shader_path.display();
    let mut fragment_shader_file = File::open(&fragment_shader_path).unwrap();
    let mut fragment_shader_src = string_from_file(&fragment_shader_file);
    let mut program = glium::Program::from_source(&display,
                                                  vertex_shader_src,
                                                  &fragment_shader_src,
                                                  None)
                          .unwrap();

    // rsnotify boilerplate to watch the fragment shader file
    let (tx, rx) = channel();
    let mut fragment_shader_watcher: RecommendedWatcher = Watcher::new(tx).unwrap();
    let _ = match fragment_shader_watcher.watch(fragment_shader_path) {
        Ok(_) => (),
        Err(e) => {
            panic!("Error watching file {}: {}",
                   fragment_shader_path_display,
                   e)
        }
    };
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
        match rx.try_recv() {
            Ok(o) => {
                fragment_shader_file = File::open(&fragment_shader_path).unwrap();
                fragment_shader_src = string_from_file(&fragment_shader_file);
                program = match glium::Program::from_source(&display,
                                                            vertex_shader_src,
                                                            &fragment_shader_src,
                                                            None) {
                    Ok(p) => p,
                    // In the error case, we print the error message
                    // and return the original program
                    Err(e) => {
                        println!("Error compiling program {}", e);
                        program
                    }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        panic!("Channel for {} watch disconnected",
                               fragment_shader_path_display)
                    }
                }
            }
        }
    }
}
