use anyhow::Result;
use glium::glutin;
use glutin::event::VirtualKeyCode;
use homework2::{get_model_matrix, get_projection_matrix, get_view_matrix};
use utils::graphic::{display_image, save_image, start_loop, Action};
use utils::rasterizer::Rasterizable;

use glam::Vec3;

mod rst;

fn main() -> Result<()> {
    let mut angle = 0.0f32;
    // let mut r = rst::Rasterizer::new(700, 700);
    let mut r = rst::Rasterizer::new(70, 70);
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

    start_loop(move |action, display| {
        let (width, height) = r.size();
        match action {
            Action::Stop => return save_image("output.png", r.data(), width, height),
            Action::Key(VirtualKeyCode::A) => angle += 10.0,
            Action::Key(VirtualKeyCode::D) => angle -= 10.0,
            _ => (),
        }
        r.clear(rst::Buffers::all());
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, rst::Primitive::Triangle);
        return display_image(&r, display);
    });
    Ok(())
}
