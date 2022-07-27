#[macro_use]
extern crate glium;

use anyhow::Result;
use glium::glutin::event::Event;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::index::PrimitiveType;
use glium::{glutin, Display, Surface};
use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use homework2::{get_model_matrix, get_projection_matrix, get_view_matrix};
use image::{save_buffer, ColorType};
use std::path::Path;
use std::time::{Duration, Instant};

use glam::Vec3;

mod rst;
mod triangle;

fn main() -> Result<()> {
    let mut angle = 0.0f32;
    let mut r = rst::Rasterizer::new(700, 700);
    let eye_pos = Vec3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vec3::new(2.0, 0.0, -2.0),
        Vec3::new(0.0, 2.0, -2.0),
        Vec3::new(-2.0, 0.0, -2.0),
        Vec3::new(3.5, -1.0, -5.0),
        Vec3::new(2.5, 1.5, -5.0),
        Vec3::new(-1.0, 0.5, -5.0),
    ];
    let ind = vec![[0, 1, 2], [3, 4, 5]];
    let cols = vec![
        Vec3::new(217.0, 238.0, 185.0),
        Vec3::new(217.0, 238.0, 185.0),
        Vec3::new(217.0, 238.0, 185.0),
        Vec3::new(185.0, 217.0, 238.0),
        Vec3::new(185.0, 217.0, 238.0),
        Vec3::new(185.0, 217.0, 238.0),
    ];
    let pos_id = r.load_positions(pos);
    let ind_id = r.load_indices(ind);
    let col_id = r.load_colors(cols);

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    start_loop(event_loop, move |action| {
        match action {
            Action::Stop => return save_image("output.png", r.data(), 700, 700),
            Action::Key(VirtualKeyCode::A) => angle += 10.0,
            Action::Key(VirtualKeyCode::D) => angle -= 10.0,
            _ => (),
        }
        r.clear(rst::Buffers::all());
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, rst::Primitive::Triangle);
        return display_image(&r, &display);
    });
    Ok(())
}

fn save_image<P: AsRef<Path>>(path: P, data: &[u8], width: u32, height: u32) -> Result<()> {
    save_buffer(path, data, width, height, ColorType::Rgb8)?;
    Ok(())
}

fn display_image(r: &rst::Rasterizer, display: &Display) -> Result<()> {
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            display,
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
        glium::IndexBuffer::new(display, PrimitiveType::TriangleStrip, &[1_u16, 2, 0, 3])?;

    let program = program!(display,
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

    let image = glium::texture::RawImage2d::from_raw_rgb_reversed(r.data(), (700, 700));
    let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(display, image)?;
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
}

fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F)
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

enum Action {
    Idle,
    Stop,
    Key(VirtualKeyCode),
}
