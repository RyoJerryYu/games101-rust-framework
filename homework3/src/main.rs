use anyhow::Result;
use glam::Vec3;
use homework3::{
    bump_fragment_shader, displacement_fragment_shader, get_model_matrix, get_projection_matrix,
    get_view_matrix, normal_fragment_shader, phong_fragment_shader, rst, shader,
    texture_fragment_shader,
};
use obj::load_obj;
use utils::{
    graphic::{save_image, Action, Control, Key},
    triangle::Triangle,
};

fn main() -> Result<()> {
    // let mut angle = 0.0;
    let mut angle = 140.0;
    let mut scale = 2.5;
    // let mut scale = 10.0;
    let filename = "output.png";
    // spot
    let obj_path = "homework3/models/spot/";
    let obj_file = "spot_triangulated_good.obj";
    let texture_file = "spot_texture.png";
    let hmap_file = "hmap.jpg";

    // rock
    // let obj_path = "homework3/models/rock/";
    // let obj_file = "rock.obj";
    // let texture_file = "rock.png";
    // let hmap_file = "";

    // cube
    // let obj_path = "homework3/models/cube/";
    // let obj_file = "cube.obj";
    // let texture_file = "";
    // let hmap_file = "";

    // crate
    // let obj_path = "homework3/models/Crate/";
    // let obj_file = "Crate1.obj";
    // let texture_file = "";
    // let hmap_file = "";

    // bunny
    // let obj_path = "homework3/models/bunny/";
    // let obj_file = "bunny.obj";
    // let texture_file = "";
    // let hmap_file = "";

    let frame_width = 700;
    let mut triangle_list = Vec::new();

    let input = std::io::BufReader::new(std::fs::File::open(format!("{}{}", obj_path, obj_file))?);
    let loadout: obj::Obj<obj::TexturedVertex> = load_obj(input)?;
    dbg!("obj loaded");

    for i in (0..loadout.indices.len()).step_by(3) {
        let mut t = Triangle::new();

        for j in 0..3 {
            // dbg!(i, j);
            let vertice = &loadout.vertices[loadout.indices[i + j] as usize];
            t.set_vertex(j, Vec3::from_array(vertice.position));
            t.set_normal(j, Vec3::from_array(vertice.normal));
            t.set_tex_coords(j, vertice.texture[0], vertice.texture[1]);
        }
        triangle_list.push(t);
    }

    dbg!("triangle_list loaded");

    let mut r = rst::Rasterizer::new(frame_width, frame_width);

    // let mut texture_path = format!("{}{}", obj_path, "rock.png");
    // r.set_texture(shader::Texture::new(&texture_path)?);

    let eye_pos = Vec3::new(0.0, 0.0, 10.0);

    r.set_vertex_shader(homework3::vertex_shader);

    // let use_shader = UseShader::Normal;
    // let use_shader = UseShader::Phong;
    // let use_shader = UseShader::Texture;
    let use_shader = UseShader::Normal;
    set_fragment_shader(&mut r, use_shader, obj_path, texture_file, hmap_file)?;

    dbg!("texture loaded");

    utils::graphic::start_loop(move |actions, display_image| {
        for action in actions {
            match action {
                Action::Stop => {
                    save_image(&r, filename)?;
                    return Ok(Control::Stop);
                }
                Action::Key(Key::A) => angle -= 10.0,
                Action::Key(Key::D) => angle += 10.0,
                Action::Key(Key::W) => scale += 0.1,
                Action::Key(Key::S) => scale -= 0.1,
                Action::Key(k)
                    if matches!(k, Key::Key1 | Key::Key2 | Key::Key3 | Key::Key4 | Key::Key5) =>
                {
                    let use_shader = match k {
                        Key::Key1 => UseShader::Normal,
                        Key::Key2 => UseShader::Phong,
                        Key::Key3 => UseShader::Texture,
                        Key::Key4 => UseShader::Bump,
                        Key::Key5 => UseShader::Displacement,
                        _ => panic!(),
                    };
                    set_fragment_shader(&mut r, use_shader, obj_path, texture_file, hmap_file)?;
                }
                _ => (),
            }
        }
        r.clear(rst::Buffers::all());

        r.set_model(get_model_matrix(angle, scale));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45., 1., 0.1, 50.));

        r.draw_triangle(&triangle_list);

        dbg!("display_image");
        display_image(&r)?;
        return Ok(Control::Continue);
    });

    return Ok(());
}

enum UseShader {
    Normal,
    Phong,
    Texture,
    Bump,
    Displacement,
}

fn set_fragment_shader(
    r: &mut rst::Rasterizer,
    use_shader: UseShader,
    obj_path: &str,
    texture_file: &str,
    hmap_file: &str,
) -> Result<()> {
    let active_shader = match use_shader {
        UseShader::Texture => {
            let texture_path = format!("{}{}", obj_path, texture_file);
            r.set_texture(shader::Texture::new(&texture_path)?);
            texture_fragment_shader
        }
        UseShader::Normal => normal_fragment_shader,
        UseShader::Phong => phong_fragment_shader,
        UseShader::Bump => {
            let texture_path = format!("{}{}", obj_path, hmap_file);
            r.set_texture(shader::Texture::new(&texture_path)?);
            bump_fragment_shader
        }
        UseShader::Displacement => {
            let texture_path = format!("{}{}", obj_path, hmap_file);
            r.set_texture(shader::Texture::new(&texture_path)?);
            displacement_fragment_shader
        }
        _ => normal_fragment_shader,
    };
    r.set_fragment_shader(active_shader);
    Ok(())
}
