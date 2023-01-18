use glam::{Vec2, Vec3};

use crate::{object::object::Object, ray::Ray, scene::Scene};

pub struct HitPayload<'a> {
    pub t_near: f32,
    pub index: usize,
    pub uv: Vec2,
    pub hit_obj: &'a Box<dyn Object>,
}

pub struct Renderer {}

fn update_progress(progress: f32) {
    let bar_width = 70;
    let pos = ((bar_width as f32 * progress) as usize).clamp(0, bar_width);
    let mut strs = String::from("[");
    strs.push_str(&String::from("=").repeat(pos));
    if pos < bar_width {
        strs.push('>');
        strs.push_str(&String::from("_").repeat(bar_width - pos - 1));
    }
    strs.push_str("] ");
    strs.push_str(&format!("{}%", (progress * 100.0) as u32));

    println!("{}", strs);
}

#[inline]
fn get_buffer_index(height: usize, width: usize, x: usize, y: usize) -> usize {
    let x = x.clamp(0, width);
    let y = y.clamp(0, height);
    y * width + x
}

impl Renderer {
    // [comment]
    // The main render function. This where we iterate over all pixels in the image, generate
    // primary rays and cast these rays into the scene. The content of the framebuffer is
    // saved to a file.
    // [/comment]
    pub fn render(&self, scene: &Scene) {
        let mut frame_buffer = vec![Vec3::ZERO; scene.width * scene.height];

        let scale = (scene.fov * 0.5).to_radians().tan();
        let image_aspect_ratio = (scene.width as f32) / (scene.height as f32);

        // Use this variable as the eye position to start your rays.
        let eye_pos = Vec3::new(-1.0, 5.0, 10.0);
        // j represent the height value, which 0 on the top
        for j in 0..scene.height {
            // i represent the width value
            for i in 0..scene.width {
                // generate primary ray direction
                // TODO: Find the x and y positions of the current pixel to get the direction
                // vector that passes through it.
                // Also, don't forget to multiply both of them with the variable *scale*, and
                // x (horizontal) variable with the *imageAspectRatio*

                // x and y is the position where ray arrived on z = -1
                // and aware that y is upside down
                let y =
                    -(j as f32 - scene.height as f32 / 2.0) / (scene.height as f32 / 2.0) * scale;
                let x = (i as f32 - scene.width as f32 / 2.0) / (scene.width as f32 / 2.0)
                    * scale
                    * image_aspect_ratio;

                let dir = Vec3::new(x, y, -1.0) // Don't forget to normalize this direction!
                    .normalize();
                let buf_index = get_buffer_index(scene.height, scene.width, i, j);
                frame_buffer[buf_index] = scene.cast_ray(&Ray::new(eye_pos, dir), 0);
            }
            update_progress((j as f32) / (scene.height as f32));
        }

        // save framebuffer to file
        let r = utils::rasterizer::BufRasterizer::from_vec3s(
            frame_buffer,
            scene.width as u32,
            scene.height as u32,
        );
        utils::graphic::save_image(&r, "output.png").expect("save image error");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_update_progress() {
        update_progress(0.0);
        update_progress(0.1);
        update_progress(0.5);
        update_progress(0.9);
        update_progress(1.0);
    }
}
