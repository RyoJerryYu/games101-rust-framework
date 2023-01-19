use anyhow::Result;
use glam::Vec3;
use homework7::light::PointLight;
use homework7::object::material::{Material, MaterialType};
use homework7::renderer::Renderer;
use homework7::scene::Scene;
use homework7::triangle::MeshTriangle;

fn main() -> Result<()> {
    let mut scene = Scene::new(784, 784);
    let mut object_holder = scene.new_object_holder();

    let red = Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.63, 0.065, 0.05),
    );
    let green = Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.14, 0.45, 0.091),
    );
    let white = Material::new(
        MaterialType::Diffuse,
        Vec3::ZERO,
        Vec3::new(0.725, 0.71, 0.68),
    );
    let light = Material::new(
        MaterialType::Diffuse,
        8.0 * Vec3::new(0.747 + 0.058, 0.747 + 0.258, 0.747)
            + 15.6 * Vec3::new(0.740 + 0.287, 0.740 + 0.160, 0.740)
            + 18.4 * Vec3::new(0.737 + 0.642, 0.737 + 0.159, 0.737),
        Vec3::ONE * 0.65,
    );
    // let liight ...?

    let floor = MeshTriangle::new("homework7/models/cornellbox/floor.obj", &white)?;
    let shortbox = MeshTriangle::new("homework7/models/cornellbox/shortbox.obj", &white)?;
    let tallbox = MeshTriangle::new("homework7/models/cornellbox/tallbox.obj", &white)?;
    let left = MeshTriangle::new("homework7/models/cornellbox/left.obj", &red)?;
    let right = MeshTriangle::new("homework7/models/cornellbox/right.obj", &green)?;
    // let light

    object_holder.add_object(Box::new(floor));
    object_holder.add_object(Box::new(shortbox));
    object_holder.add_object(Box::new(tallbox));
    object_holder.add_object(Box::new(left));
    object_holder.add_object(Box::new(right));

    // no add light, add object light
    scene.build_bvh(object_holder);

    let r = Renderer {};
    r.render(&scene);

    Ok(())
}
