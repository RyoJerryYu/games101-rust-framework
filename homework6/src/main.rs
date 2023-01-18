use glam::Vec3;
use homework6::{light::PointLight, renderer::Renderer, scene::Scene, triangle::MeshTriangle};

fn main() {
    let mut scene = Scene::new(1280, 960);
    let mut object_holder = scene.new_object_holder();

    let bunny =
        MeshTriangle::new("homework3/models/bunny/bunny.obj").expect("load bunny.obj failed");

    object_holder.add_object(Box::new(bunny));
    scene.add_light(Box::new(PointLight::new(
        &Vec3::new(-20.0, 70.0, 20.0),
        1.0,
    )));
    scene.add_light(Box::new(PointLight::new(&Vec3::new(20.0, 70.0, 20.0), 1.0)));

    scene.build_bvh(object_holder);

    let r = Renderer {};
    r.render(&scene);
}
