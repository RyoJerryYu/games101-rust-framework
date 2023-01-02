use anyhow::Result;
use glam::Vec3;
use glium::glutin::event::VirtualKeyCode;
use homework3::{
    bump_fragment_shader, displacement_fragment_shader, get_model_matrix, get_projection_matrix,
    get_view_matrix, normal_fragment_shader, phong_fragment_shader, rst,
    shader::{self, FragmentShader},
    texture_fragment_shader,
};
use obj::load_obj;
use utils::{
    graphic::{display_image, save_image, Action},
    triangle::Triangle,
};

fn main() -> Result<()> {
    let mut angle = 140.0;
    let filename = "input.png";
    let obj_path = "homework3/models/cube/";
    let mut triangle_list = Vec::new();

    let input =
        std::io::BufReader::new(std::fs::File::open(format!("{}{}", obj_path, "cube.obj"))?);
    let loadout: obj::Obj<obj::TexturedVertex> = load_obj(input)?;
    dbg!("obj loaded");

    for i in (0..loadout.indices.len()).step_by(3) {
        let mut t = Triangle::new();

        for j in 0..3 {
            dbg!(i, j);
            let vertice = loadout.vertices[loadout.indices[i + j] as usize];
            t.set_vertex(j, Vec3::from_array(vertice.position));
            t.set_normal(j, Vec3::from_array(vertice.normal));
            t.set_tex_coords(j, vertice.texture[0], vertice.texture[1]);
            triangle_list.push(t);
        }
    }

    dbg!("triangle_list loaded");

    let mut r = rst::Rasterizer::new(700, 700);

    let mut texture_path = format!("{}{}", obj_path, "rock.png");
    // r.set_texture(shader::Texture::new(&texture_path)?);

    dbg!("texture loaded");

    let mut active_shader: FragmentShader = phong_fragment_shader;

    let use_shader = "normal";
    match use_shader {
        "texture" => {
            active_shader = texture_fragment_shader;
            texture_path = format!("{}{}", obj_path, "spot_texture.png");
            r.set_texture(shader::Texture::new(&texture_path)?);
        }
        "normal" => active_shader = normal_fragment_shader,
        "phong" => active_shader = phong_fragment_shader,
        "bump" => active_shader = bump_fragment_shader,
        "displacement" => active_shader = displacement_fragment_shader,
        _ => (),
    }

    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    r.set_vertex_shader(homework3::vertex_shader);
    r.set_fragment_shader(active_shader);

    utils::graphic::start_loop(move |action, display| {
        match action {
            Action::Stop => return save_image(&r, filename),
            Action::Key(VirtualKeyCode::A) => angle += 10.0,
            Action::Key(VirtualKeyCode::D) => angle -= 10.0,
            _ => (),
        }
        r.clear(rst::Buffers::all());

        r.set_model(get_model_matrix(angle));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45., 1., 0.1, 50.));

        r.draw_triangle(&triangle_list);

        dbg!("display_image");
        return display_image(&r, display);
    });

    return Ok(());
}
