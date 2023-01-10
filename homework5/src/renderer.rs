use glam::{Vec2, Vec3};

use crate::{object, scene};

pub struct HitPayload {
    pub t_near: f32,
    pub index: u32,
    pub uv: Vec2,
    pub hit_obj: Box<dyn object::Object>,
}

pub struct Renderer {}

#[inline]
fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - 2.0 * i.dot(n) * n
}

// [comment]
// Compute refraction direction using Snell's law
//
// We need to handle with care the two possible situations:
//
//    - When the ray is inside the object
//
//    - When the ray is outside.
//
// If the ray is outside, you need to make cosi positive cosi = -N.I
//
// If the ray is inside, you need to invert the refractive indices and negate the normal N
// [/comment]
fn reflect_with_ior(i: Vec3, n: Vec3, ior: f32) -> Vec3 {
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    let mut n = n;
    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        (etai, etat) = (etat, etai);
        n = -n;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3::ZERO
    } else {
        eta * i + (eta * cosi - k.sqrt()) * n
    }
}

// [comment]
// Compute Fresnel equation
//
// \param I is the incident view direction
//
// \param N is the normal at the intersection point
//
// \param ior is the material refractive index
// [/comment]
fn fresnel(i: Vec3, n: Vec3, ior: f32) -> f32 {
    let mut cosi = i.dot(n).clamp(-1.0, 1.0);
    let (mut etai, mut etat) = (1.0, ior);
    if cosi > 0.0 {
        (etai, etat) = (etat, etai);
    }
    // Compute sini using Snell's law
    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    // Total internal reflection
    if sint >= 1.0 {
        return 1.0;
    }

    let cost = (1.0 - sint * sint).max(0.0).sqrt();
    cosi = cosi.abs();
    let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
    let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
    return (rs * rs + rp * rp) / 2.0;
    // As a consequence of the conservation of energy, transmittance is given by:
    // kt = 1 - kr;
}

// [comment]
// Returns true if the ray intersects an object, false otherwise.
//
// \param orig is the ray origin
// \param dir is the ray direction
// \param objects is the list of objects the scene contains
// \param[out] tNear contains the distance to the cloesest intersected object.
// \param[out] index stores the index of the intersect triangle if the interesected object is a mesh.
// \param[out] uv stores the u and v barycentric coordinates of the intersected point
// \param[out] *hitObject stores the pointer to the intersected object (used to retrieve material information, etc.)
// \param isShadowRay is it a shadow ray. We can return from the function sooner as soon as we have found a hit.
// [/comment]
fn trace(orig: Vec3, dir: Vec3, objects: &Vec<Box<dyn object::Object>>) -> Option<HitPayload> {
    todo!()
}

// [comment]
// Implementation of the Whitted-style light transport algorithm (E [S*] (D|G) L)
//
// This function is the function that compute the color at the intersection point
// of a ray defined by a position and a direction. Note that thus function is recursive (it calls itself).
//
// If the material of the intersected object is either reflective or reflective and refractive,
// then we compute the reflection/refraction direction and cast two new rays into the scene
// by calling the castRay() function recursively. When the surface is transparent, we mix
// the reflection and refraction color using the result of the fresnel equations (it computes
// the amount of reflection and refraction depending on the surface normal, incident view direction
// and surface refractive index).
//
// If the surface is diffuse/glossy we use the Phong illumation model to compute the color
// at the intersection point.
// [/comment]
fn castRay(orig: Vec3, dir: Vec3, scene: &scene::Scene, depth: i32) -> Vec3 {
    todo!()
}

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
fn get_buffer_index(height: usize, width: usize, x: f32, y: f32) -> usize {
    let x = (x as usize).clamp(0, width);
    let y = (y as usize).clamp(0, height);
    y * width + x
}

impl Renderer {
    // [comment]
    // The main render function. This where we iterate over all pixels in the image, generate
    // primary rays and cast these rays into the scene. The content of the framebuffer is
    // saved to a file.
    // [/comment]
    pub fn render(&self, scene: &scene::Scene) {
        let mut frame_buffer = vec![Vec3::ZERO; scene.width * scene.height];
        let scale = (scene.fov * 0.5).to_radians().tan();
        let image_aspect_ratio = (scene.width as f32) / (scene.height as f32);

        // Use this variable as the eye position to start your rays.
        let mut eye_pos = Vec3::ZERO;
        // j represent the height value
        for j in 0..scene.height {
            // i represent the width value
            for i in 0..scene.width {
                let x = 0.0;
                let y = 0.0;
                // TODO: Find the x and y positions of the current pixel to get the direction
                // vector that passes through it.
                // Also, don't forget to multiply both of them with the variable *scale*, and
                // x (horizontal) variable with the *imageAspectRatio*

                let dir = Vec3::new(x, y, -1.0); // Don't forget to normalize this direction!
                let buf_index = get_buffer_index(scene.height, scene.width, x, y);
                frame_buffer[buf_index] = castRay(eye_pos, dir, scene, 0);
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
