use anyhow::Result;

use glium::{
    glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    glutin::window::WindowBuilder,
    glutin::{
        event_loop::{ControlFlow, EventLoop},
        ContextBuilder,
    },
    index::PrimitiveType,
    texture::{CompressedSrgbTexture2d, RawImage2d},
    IndexBuffer, Program, Surface,
};
use image::{save_buffer, ColorType};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::rst;

pub fn save_image<P: AsRef<Path>>(path: P, data: &[u8], width: u32, height: u32) -> Result<()> {
    save_buffer(path, data, width, height, ColorType::Rgb8)?;
    Ok(())
}

pub enum Action {
    Idle,
    Stop,
    Key(VirtualKeyCode),
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)
where
    F: 'static + FnMut(&Action) -> Result<()>,
{
    event_loop.run(move |event, _, ctrl_flow| {
        *ctrl_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_nanos(16_666_667));

        let action = match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => Action::Stop,
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => match virtual_code {
                VirtualKeyCode::Escape => Action::Stop,
                _ => Action::Key(virtual_code),
            },
            _ => Action::Idle,
        };

        callback(&action).expect("rendering failed");
        match action {
            Action::Stop => *ctrl_flow = ControlFlow::Exit,
            _ => (),
        }
    })
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct ImageDisplayer {
    display: glium::Display,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: Program,
}

impl ImageDisplayer {
    pub fn new<E>(events_loop: &EventLoop<E>) -> Result<Self> {
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, events_loop)?;
        let vertex_buffer = {
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
            )?
        };

        let index_buffer =
            IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1_u16, 2, 0, 3])?;

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
        )?;

        Ok(Self {
            display,
            vertex_buffer,
            index_buffer,
            program,
        })
    }

    pub fn display_image(&self, r: &rst::Rasterizer, wh: (u32, u32)) -> Result<()> {
        let image = RawImage2d::from_raw_rgb_reversed(r.data(), wh);
        let opengl_texture = CompressedSrgbTexture2d::new(&self.display, image)?;
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &opengl_texture,
        };
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default(),
        )?;
        target.finish()?;
        Ok(())
    }
}
