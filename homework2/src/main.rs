use anyhow::Result;
use homework2::{get_model_matrix, get_projection_matrix, get_view_matrix};
use utils::graphic::{save_image, start_loop, Action, Control, Key};

use glam::Vec3;

mod rst;

fn main() -> Result<()> {
    let mut angle = 0.0f32;
    let mut r = rst::Rasterizer::new(700, 700);
    // let mut r = rst::Rasterizer::new(20, 20);
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

    start_loop(move |actions, display_image| {
        for action in actions {
            match action {
                Action::Stop => {
                    save_image(&r, "output.png")?;
                    return Ok(Control::Stop);
                }
                Action::Key(Key::A) => angle += 10.0,
                Action::Key(Key::D) => angle -= 10.0,
                _ => (),
            }
        }
        r.clear(rst::Buffers::all());
        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, rst::Primitive::Triangle);
        display_image(&r)?;
        Ok(Control::Continue)
    });
    Ok(())
}
