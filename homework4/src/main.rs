use anyhow::Ok;
use glam::Vec2;
use homework4::rst;
use utils::graphic::{save_image, Action, Control};

fn main() {
    let frame_width = 700;
    let path = "output.png";

    let mut r = rst::Rasterizer::new(frame_width, frame_width);

    let mut cursor_at: (f32, f32) = (0.0, 0.0);
    let mut control_points = vec![];

    utils::graphic::start_loop(frame_width, frame_width, move |actions, display_image| {
        for action in actions {
            match action {
                Action::Stop => {
                    save_image(&r, path)?;
                    return Ok(Control::Stop);
                }
                // use move + click for get clicked point
                Action::Move { x, y } => cursor_at = (*x as f32, *y as f32),
                Action::Clicked => {
                    if control_points.len() >= 4 {
                        control_points.clear();
                    }
                    control_points.push(Vec2 {
                        x: cursor_at.0,
                        y: cursor_at.1,
                    });
                    dbg!(&control_points);
                }
                _ => (),
            }
        }

        r.clear(rst::Buffers::all());
        for p in &control_points {
            r.draw_circle(*p, 10.0);
        }
        // if len > 4 , draw bezier
        display_image(&r)?;

        Ok(Control::Continue)
    });
}
