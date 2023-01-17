use std::ops::{Mul, Neg};

use glam::{Vec2, Vec3};

use crate::{object::object::Object, ray::Ray, scene::Scene};

pub struct HitPayload<'a> {
    pub t_near: f32,
    pub index: usize,
    pub uv: Vec2,
    pub hit_obj: &'a Box<dyn Object>,
}

pub struct Renderer {}

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
// fn trace<'a>(
//     orig: &Vec3,
//     dir: &Vec3,
//     objects: &'a Vec<Box<dyn Object>>,
// ) -> Option<HitPayload<'a>> {
//     let mut tnear = f32::INFINITY;
//     let mut payload: Option<HitPayload> = None;

//     for obj in objects {
//         let mut tneark = f32::INFINITY;
//         let mut indexk = 0;
//         let mut uvk = Vec2::ZERO;

//         if obj.intersect(&orig, &dir, &mut tneark, &mut indexk, &mut uvk) && tneark < tnear {
//             payload = Some(HitPayload {
//                 t_near: tneark,
//                 index: indexk,
//                 uv: uvk,
//                 hit_obj: obj,
//             });
//             tnear = tneark;
//         }
//     }

//     return payload;
// }

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
// fn cast_ray(orig: &Vec3, dir: &Vec3, scene: &scene::Scene, depth: u32) -> Vec3 {
//     if depth > scene.max_depth {
//         return Vec3::ZERO;
//     }

//     let mut hit_color = scene.background_color;
//     let payload = trace(orig, dir, scene.get_objects());

//     let payload = if let Some(payload) = payload {
//         payload
//     } else {
//         return scene.background_color;
//     };

//     let hit_point = *orig + *dir * payload.t_near;
//     let mut n = Vec3::ZERO; // normal
//     let mut st = Vec2::ZERO; // st coordinates
//     payload.hit_obj.get_surface_properties(
//         &hit_point,
//         dir,
//         &payload.index,
//         &payload.uv,
//         &mut n,
//         &mut st,
//     );

//     let obj_payload = payload.hit_obj.get_render_payload();

//     match obj_payload.material_type {
//         MaterialType::ReflectionAndRefraction => {
//             let reflection_direction = reflect(*dir, n).normalize();
//             let refraction_direction = refract(*dir, n, obj_payload.ior).normalize();

//             let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
//                 hit_point - n * scene.epsilon
//             } else {
//                 hit_point + n * scene.epsilon
//             };
//             let refraction_ray_orig = if refraction_direction.dot(n) < 0.0 {
//                 hit_point - n * scene.epsilon
//             } else {
//                 hit_point + n * scene.epsilon
//             };

//             let reflection_color = cast_ray(
//                 &reflection_ray_orig,
//                 &reflection_direction,
//                 scene,
//                 depth + 1,
//             );
//             let refraction_color = cast_ray(
//                 &refraction_ray_orig,
//                 &refraction_direction,
//                 scene,
//                 depth + 1,
//             );

//             let kr = fresnel(*dir, n, obj_payload.ior);
//             hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
//         }
//         MaterialType::Reflection => {
//             let kr = fresnel(*dir, n, obj_payload.ior);
//             let reflection_direction = reflect(*dir, n);
//             let reflection_ray_orig = if reflection_direction.dot(n) < 0.0 {
//                 hit_point + n * scene.epsilon
//             } else {
//                 hit_point - n * scene.epsilon
//             };
//             hit_color = cast_ray(
//                 &reflection_ray_orig,
//                 &reflection_direction,
//                 scene,
//                 depth + 1,
//             ) * kr;
//         }
//         _ => {
//             // [comment]
//             // We use the Phong illumation model int the default case. The phong model
//             // is composed of a diffuse and a specular reflection component.
//             // [/comment]
//             let mut light_amt = Vec3::ZERO;
//             let mut specular_color = Vec3::ZERO;
//             let shadow_point_orig = if dir.dot(n) < 0.0 {
//                 hit_point + n * scene.epsilon
//             } else {
//                 hit_point - n * scene.epsilon
//             };

//             // [comment]
//             // Loop over all lights in the scene and sum their contribution up
//             // We also apply the lambert cosine law
//             // [/comment]
//             for light in scene.get_lights() {
//                 let light_dir = light.position() - hit_point;
//                 // square of the distance between hitPoint and the light
//                 let light_distance_2 = light_dir.dot(light_dir);
//                 let light_dir = light_dir.normalize();
//                 let l_dot_n = light_dir.dot(n).max(0.0);
//                 // is the point in shadow, and is the nearest occluding object closer to the object than the light itself?
//                 let shadow_res = trace(&shadow_point_orig, &light_dir, scene.get_objects());
//                 let in_shadow = match shadow_res {
//                     Some(shadow_res) => shadow_res.t_near * shadow_res.t_near < light_distance_2,
//                     None => false,
//                 };

//                 light_amt = match in_shadow {
//                     true => light_amt + Vec3::ZERO,
//                     false => light_amt + light.intensity() * l_dot_n,
//                 };

//                 let reflection_direction = reflect(-light_dir, n);
//                 specular_color += reflection_direction
//                     .dot(*dir)
//                     .neg()
//                     .max(0.0)
//                     .powf(obj_payload.specular_exponent)
//                     .mul(light.intensity());
//             }

//             hit_color = light_amt * payload.hit_obj.eval_diffuse_color(&st) * obj_payload.kd
//                 + specular_color * obj_payload.ks;
//         }
//     }

//     return hit_color;
// }

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
