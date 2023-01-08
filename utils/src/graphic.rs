use anyhow::Result;
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event_loop::ControlFlow;
use glium::index::PrimitiveType;
use glium::{glutin, implement_vertex, program, uniform, Surface};
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event::{Event, MouseButton, StartCause};
use image::{save_buffer, ColorType};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::rasterizer;

pub fn save_image<P: AsRef<Path>>(rst: &impl rasterizer::Rasterizable, path: P) -> Result<()> {
    let data = rst.data();
    let (width, height) = rst.size();
    save_buffer(path, data, width, height, ColorType::Rgb8)?;
    Ok(())
}

type DisplayImage = Box<dyn Fn(&dyn rasterizer::Rasterizable) -> Result<()>>;

pub fn start_loop<F>(width: u32, height: u32, mut callback: F)
where
    F: 'static + FnMut(&Vec<Action>, &DisplayImage) -> Result<Control>,
{
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(PhysicalSize::new(width,height));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1_u16, 2, 0, 3]).unwrap();

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec2 tex_coords;
                varying vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 110
                uniform sampler2D tex;
                varying vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec2 tex_coords;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 100
                uniform lowp sampler2D tex;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },
    )
    .unwrap();

    let display_image: DisplayImage =
        Box::new(move |rst: &dyn rasterizer::Rasterizable| -> Result<()> {
            let image = glium::texture::RawImage2d::from_raw_rgb_reversed(rst.data(), rst.size());
            let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&display, image)?;
            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                tex: &opengl_texture
            };
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )?;
            target.finish()?;
            Ok(())
        });

    let mut action_buffer = Vec::new();
    let mut next_frame_time = Instant::now();

    event_loop.run(move |event, _, ctrl_flow| {
        *ctrl_flow = ControlFlow::WaitUntil(next_frame_time);

        let action = match event {
            Event::NewEvents(cause) => match cause {
                // the event start caouse,
                // calling the callback, and decide whether to stop or continue
                StartCause::ResumeTimeReached { .. } | StartCause::Init => {
                    *ctrl_flow = match callback(&action_buffer, &display_image) {
                        Ok(Control::Continue) => {
                            action_buffer.clear();
                            next_frame_time = Instant::now() + Duration::from_nanos(16666667);
                            ControlFlow::WaitUntil(next_frame_time)
                        }
                        Ok(Control::Stop) => ControlFlow::Exit,
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            ControlFlow::Exit
                        }
                    };
                    return;
                }

                // ignore other start caouse action
                _ => Action::Idle,
            },

            Event::WindowEvent { event, .. } => match event {
                // close window is a stop action
                WindowEvent::CloseRequested => Action::Stop,

                // when keyboard pressed should be a key action
                // except `esc` , that should be a stop action
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match virtual_code {
                    VirtualKeyCode::Escape => Action::Stop,
                    _ => Action::Key(virtual_code),
                },

                // when cursor moved, should be a move action
                WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y },
                    ..
                } => Action::Move { x, y },

                // when mouse clicked, should be a click action
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => Action::Clicked,
                _ => Action::Idle,
            },
            _ => Action::Idle,
        };

        if !matches!(action, Action::Idle) {
            dbg!(&action);
        }

        action_buffer.push(action);
    })
}

pub type Key = VirtualKeyCode;

#[derive(Debug)]
pub enum Action {
    Idle,
    Stop,
    Key(Key),
    Move { x: f64, y: f64 },
    Clicked,
}

pub enum Control {
    Continue,
    Stop,
}
