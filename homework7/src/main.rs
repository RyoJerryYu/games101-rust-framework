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

    // let mut red = Material::new(MaterialType::DiffuseAndGlossy, color, emission);
    // red.kd =
    // let green
    // let white
    // let liight ...?

    let floor = MeshTriangle::new("homework7/models/cornellbox/floor.obj")?;
    let shortbox = MeshTriangle::new("homework7/models/cornellbox/shortbox.obj")?;
    let tallbox = MeshTriangle::new("homework7/models/cornellbox/tallbox.obj")?;
    let left = MeshTriangle::new("homework7/models/cornellbox/left.obj")?;
    let right = MeshTriangle::new("homework7/models/cornellbox/right.obj")?;
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
