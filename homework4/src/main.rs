use anyhow::Ok;
use glam::Vec2;
use homework4::{bezier, naive_bezier, rst};
use utils::graphic::{save_image, Action, Control};

fn main() {
    let frame_width = 700;
    let path = "output.png";

    let mut r = rst::Rasterizer::new(frame_width, frame_width);

    let mut control_points = vec![];

    utils::graphic::start_loop(frame_width, frame_width, move |actions, display_image| {
        for action in actions {
            match action {
                Action::Stop => {
                    save_image(&r, path)?;
                    return Ok(Control::Stop);
                }
                // use move + click for get clicked point
                Action::Clicked { x, y } => {
                    if control_points.len() >= 4 {
                        control_points.clear();
                    }
                    control_points.push(Vec2 { x: *x, y: *y });
                    dbg!(&control_points);
                }
                _ => (),
            }
        }

        r.clear(rst::Buffers::all());
        for p in &control_points {
            r.draw_circle(*p, 10.0);
        }
        if control_points.len() >= 4 {
            naive_bezier(&mut r, &control_points);
            bezier(&mut r, &control_points);
        }
        display_image(&r)?;

        Ok(Control::Continue)
    });
}
