use glam::{Vec2, Vec3};
use homework5::{light, object::MaterialType, scene, sphere, triangle, renderer};

fn main() {
    let mut scene = scene::Scene::new(1280, 960);

    let mut sph1 = sphere::Sphere::new(&Vec3::new(-1.0, 0.0, -12.0), 2.0);
    // sph1.render_payload.material_type = MaterialType::DiffuseAndGlossy;
    sph1.render_payload.material_type = MaterialType::DiffuseAndGlossy;
    sph1.render_payload.diffuse_color = Vec3::new(0.6, 0.7, 0.8);

    let mut sph2 = sphere::Sphere::new(&Vec3::new(0.5, -0.5, -8.0), 1.5);
    sph2.render_payload.ior = 1.5;
    sph2.render_payload.material_type = MaterialType::ReflectionAndRefraction;

    scene.add_object(Box::new(sph1));
    scene.add_object(Box::new(sph2));

    // add mesh triangle
    let verts = vec![
        Vec3::new(-5.0, -3.0, -6.0),
        Vec3::new(5.0, -3.0, -6.0),
        Vec3::new(5.0, -3.0, -16.0),
        Vec3::new(-5.0, -3.0, -16.0),
    ];
    let vert_index = vec![0, 1, 3, 1, 2, 3];
    let st = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];
    let mut mesh = triangle::MeshTriangle::new(verts, vert_index, 2, st);
    mesh.render_payload.material_type = MaterialType::DiffuseAndGlossy;
    scene.add_object(Box::new(mesh));

    scene.add_light(Box::new(light::PointLight::new(
        &Vec3::new(-20.0, 70.0, 20.0),
        0.5,
    )));
    scene.add_light(Box::new(light::PointLight::new(
        &Vec3::new(30.0, 50.0, -12.0),
        0.5,
    )));
    let r = renderer::Renderer{};
    r.render(&scene)
}
