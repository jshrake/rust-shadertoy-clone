#[macro_use]
extern crate glium;
extern crate notify;
extern crate time;

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

fn shadertoy_fragment_shader(src: &str) -> String {
    let header = r#"
        #version 140
        uniform vec3 iResolution;
        uniform float iGlobalTime;
        uniform float iTimeDelta;
        uniform float iGlobalFrame;
        uniform vec4 iMouse;
        out vec4 fragColor;
    "#;
    let footer = r#"
        void main() {
            mainImage(fragColor, gl_FragCoord.xy);
        }
    "#;
    format!("{header}\n{src}\n{footer}",
            header = header,
            src = src,
            footer = footer)
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                      .with_title("shdrs")
                      .with_vsync()
                      .build_glium()
                      .unwrap();

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
    let mut program = {
        let fragment_shader_file = File::open(&fragment_shader_path).unwrap();
        let fragment_shader_src = string_from_file(&fragment_shader_file);
        let shadertoy_fragment_shader_src = shadertoy_fragment_shader(&fragment_shader_src);
        glium::Program::from_source(&display,
                                    vertex_shader_src,
                                    &shadertoy_fragment_shader_src,
                                    None)
            .unwrap()
    };

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
    #[derive(Debug, Clone, Copy)]
    struct MouseState {
        x: f32,
        y: f32,
        left_button_pressed: bool,
    };
    let mut mouse_last_state = MouseState {
        x: 0.0,
        y: 0.0,
        left_button_pressed: false,
    };
    let mut mouse_current_state = MouseState {
        x: 0.0,
        y: 0.0,
        left_button_pressed: false,
    };
    let mut frame_count = 0;
    let mut u_mouse = [0.0, 0.0, 0.0, 0.0];
    let start_time = time::SteadyTime::now();
    let mut last_time = start_time;
    loop {
        let mut target = display.draw();
        let (target_width, target_height) = target.get_dimensions();
        let uniforms = {
            // Update the screen resolution
            let u_resolution = {
                let w = target_width as f32;
                let h = target_height as f32;
                let ar = h / w;
                [w, h, ar]
            };
            // Update the mouse uniform
            if mouse_current_state.left_button_pressed {
                u_mouse[0] = mouse_current_state.x;
                u_mouse[1] = mouse_current_state.y;
            }
            if mouse_current_state.left_button_pressed != mouse_last_state.left_button_pressed {
                if mouse_current_state.left_button_pressed {
                    u_mouse[2] = mouse_current_state.x;
                    u_mouse[3] = mouse_current_state.y;
                } else {
                    u_mouse[2] = -mouse_current_state.x;
                    u_mouse[3] = -mouse_current_state.y;
                }
            }
            // Time uniforms
            let current_time = time::SteadyTime::now();
            let frame_delta = current_time - last_time;
            last_time = current_time;
            let run_time = current_time - start_time;
            let u_global_time = run_time.num_milliseconds() as f32 / 1000.0;
            let u_time_delta = frame_delta.num_milliseconds() as f32 / 1000.0;

            uniform! {
                iResolution: u_resolution,
                iGlobalFrame: frame_count,
                iMouse: u_mouse,
                iGlobalTime: u_global_time,
                iTimeDelta: u_time_delta,
            }
        };
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();
        frame_count += 1;
        // Update mouse state
        mouse_last_state = mouse_current_state;
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x, y) => {
                    mouse_current_state.x = x as f32;
                    mouse_current_state.y = (target_height as i32 - y) as f32;
                }
                glium::glutin::Event::MouseInput(state, button) => {
                    match button {
                        glium::glutin::MouseButton::Left => {
                            match state {
                                glium::glutin::ElementState::Pressed => {
                                    mouse_current_state.left_button_pressed = true;
                                }
                                glium::glutin::ElementState::Released => {
                                    mouse_current_state.left_button_pressed = false;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        // Recompile program on fragment shader file changes
        match rx.try_recv() {
            Ok(_) => {
                program = {
                    let fragment_shader_file = File::open(&fragment_shader_path).unwrap();
                    let fragment_shader_src = string_from_file(&fragment_shader_file);
                    let shadertoy_fragment_shader_src =
                        shadertoy_fragment_shader(&fragment_shader_src);
                    match glium::Program::from_source(&display,
                                                      vertex_shader_src,
                                                      &shadertoy_fragment_shader_src,
                                                      None) {
                        Ok(p) => p,
                        // In the error case, we print the error message
                        // and return the original program
                        Err(e) => {
                            println!("Error compiling program {}", e);
                            program
                        }
                    }
                };
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
