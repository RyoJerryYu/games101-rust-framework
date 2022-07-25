#[macro_use]
extern crate glium;

use anyhow::Result;
use glium::glutin;
use glutin::event::VirtualKeyCode;
use homework1::{get_model_matrix, get_projection_matrix, get_view_matrix};

use display::Action;
use glam::Vec3;

mod display;
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
    ];
    let ind = vec![[0, 1, 2]];
    let pos_id = r.load_positions(pos);
    let ind_id = r.load_indices(ind);

    let event_loop = glutin::event_loop::EventLoop::new();
    let displayer = display::ImageDisplayer::new(&event_loop)?;

    display::start_loop(event_loop, move |action| {
        match action {
            Action::Stop => return display::save_image("output.png", r.data(), 700, 700),
            Action::Key(VirtualKeyCode::A) => angle += 10.0,
            Action::Key(VirtualKeyCode::D) => angle -= 10.0,
            _ => (),
        }
        r.clear(rst::Buffers::all());
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, rst::Primitive::Triangle);
        displayer.display_image(&r, (700, 700))
    });
    Ok(())
}
