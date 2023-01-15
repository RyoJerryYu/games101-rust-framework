use glam::Vec3;
use homework6::{
    light::{Light, PointLight},
    renderer::Renderer,
    scene::Scene,
    triangle::MeshTriangle,
};

fn main() {
    let mut scene = Scene::new(1280, 960);

    let bunny = MeshTriangle::new("bunny.obj");

    scene.add_object(Box::new(bunny));
    scene.add_light(Box::new(PointLight::new(
        &Vec3::new(-20.0, 70.0, 70.0),
        1.0,
    )));
    scene.add_light(Box::new(PointLight::new(&Vec3::new(20.0, 70.0, 70.0), 1.0)));

    scene.build_bvh();

    let r = Renderer {};
    r.render(&scene);
}
