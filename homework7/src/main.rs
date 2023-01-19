use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use glam::Vec3;
use homework7::object::material::{Material, MaterialType};
use homework7::renderer::Renderer;
use homework7::scene::Scene;
use homework7::triangle::MeshTriangle;

fn main() -> Result<()> {
    let mut scene = Scene::new(784, 784);

    let red = Arc::new(Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.63, 0.065, 0.05),
    ));
    let green = Arc::new(Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.14, 0.45, 0.091),
    ));
    let white = Arc::new(Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.725, 0.71, 0.68),
    ));
    let light = Arc::new(Material::new(
        MaterialType::Diffuse,
        8.0 * Vec3::new(0.747 + 0.058, 0.747 + 0.258, 0.747)
            + 15.6 * Vec3::new(0.740 + 0.287, 0.740 + 0.160, 0.740)
            + 18.4 * Vec3::new(0.737 + 0.642, 0.737 + 0.159, 0.737),
        Vec3::ONE * 0.65,
    ));
    // let liight ...?

    let floor = MeshTriangle::new("homework7/models/cornellbox/floor.obj", white.clone())?;
    let shortbox = MeshTriangle::new("homework7/models/cornellbox/shortbox.obj", white.clone())?;
    let tallbox = MeshTriangle::new("homework7/models/cornellbox/tallbox.obj", white.clone())?;
    let left = MeshTriangle::new("homework7/models/cornellbox/left.obj", red.clone())?;
    let right = MeshTriangle::new("homework7/models/cornellbox/right.obj", green.clone())?;
    let light_obj = MeshTriangle::new("homework7/models/cornellbox/light.obj", light.clone())?;

    scene.add_object(Arc::new(floor));
    scene.add_object(Arc::new(shortbox));
    scene.add_object(Arc::new(tallbox));
    scene.add_object(Arc::new(left));
    scene.add_object(Arc::new(right));
    scene.add_object(Arc::new(light_obj));

    // no add light, add object light
    scene.build_bvh();

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 1,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 2,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 4,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 8,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 16,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 64,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );

    let r = Renderer {
        prefix: String::from("fixed"),
        spp: 1024,
    };
    let start_time = Instant::now();
    r.render(&scene);
    println!(
        "rendering(spp={}) cost {} seconds",
        r.spp,
        start_time.elapsed().as_secs_f32()
    );
    Ok(())
}
