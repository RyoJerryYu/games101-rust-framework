use anyhow::Result;
use glam::{Vec2, Vec3};

pub struct Texture {
    pub width: u32,
    pub height: u32,
    img: image::RgbImage,
}

impl Texture {
    pub fn new(name: &str) -> Result<Self> {
        let img = image::io::Reader::open(name)?.decode()?;
        let img = img.to_rgb8();
        let width = img.width();
        let height = img.height();
        Ok(Self { width, height, img })
    }

    /**
     * from the left bottom corner
     * - u: horizontal
     * - v: vertical
     */
    pub fn get_color(&self, u: f32, v: f32) -> utils::triangle::Rgb {
        let u_img = u * self.width as f32;
        let u_img = (u_img as u32).clamp(0, self.width - 1);
        let v_img = (1.0 - v) * self.height as f32;
        let v_img = (v_img as u32).clamp(0, self.height - 1);
        let color = self.img.get_pixel(u_img, v_img);
        utils::triangle::Rgb::from(color)
    }

    pub fn get_color_bilinear(&self, u: f32, v: f32) -> utils::triangle::Rgb {
        let u_img = u * self.width as f32;
        let v_img = (1.0 - v) * self.height as f32;
        let u_lo = (u_img as u32).clamp(0, self.width - 1);
        let u_hi = (u_lo + 1).clamp(0, self.width - 1);
        let v_lo = (v_img as u32).clamp(0, self.height - 1);
        let v_hi = (v_lo + 1).clamp(0, self.height - 1);

        let u_t = u_img - u_lo as f32;
        let v_t = v_img - v_lo as f32;
        let u_lo_hi = [u_lo, u_hi];
        let v_lo_hi = [v_lo, v_hi];

        let mut color_corner = vec![vec![utils::triangle::Rgb::default(); 2]; 2];

        for u_ind in 0..=1 {
            for v_ind in 0..=1 {
                color_corner[u_ind][v_ind] =
                    self.img.get_pixel(u_lo_hi[u_ind], v_lo_hi[v_ind]).into();
            }
        }

        let res_ulo = lerp_rgb(color_corner[0][0], color_corner[0][1], v_t);
        let res_uhi = lerp_rgb(color_corner[1][0], color_corner[1][1], v_t);
        let res = lerp_rgb(res_ulo, res_uhi, u_t);

        res
    }
}

fn lerp_rgb(v0: utils::triangle::Rgb, v1: utils::triangle::Rgb, t: f32) -> utils::triangle::Rgb {
    return utils::triangle::Rgb(
        ((v0.0 as f32) * (1.0 - t) + (v1.0 as f32) * t) as u8,
        ((v0.1 as f32) * (1.0 - t) + (v1.1 as f32) * t) as u8,
        ((v0.2 as f32) * (1.0 - t) + (v1.2 as f32) * t) as u8,
    );
}

pub struct FragmentShaderPayload<'a> {
    pub view_pos: Vec3,
    pub color: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub texture: &'a Option<Texture>,
}

pub type FragmentShader = fn(&FragmentShaderPayload) -> Vec3;

pub struct VertexShaderPayload {
    pub position: Vec3,
}

pub type VertexShader = fn(&VertexShaderPayload) -> Vec3;

mod test {

    #[test]
    fn test_texture() {
        let texture = super::Texture::new("output.png").unwrap();
        let color = texture.get_color(0.5, 0.35);
        println!("{:?}", color);
    }

    #[test]
    fn test_testure_raw() {
        use utils::triangle::Rgb;
        /*
         * black -> blue
         * |         |
         * v         v
         * green -> cyan
         */
        let raw: &[u8] = &[
            0, 0, 0, //
            0, 0, 128, //
            0, 0, 255, //
            0, 128, 0, //
            0, 128, 128, //
            0, 128, 255, //
            0, 255, 0, //
            0, 255, 128, //
            0, 255,
            255, //

                 // 255,0,0,
                 // 255,0,128,
                 // 255,0,255,
        ];

        let r = utils::rasterizer::BufRasterizer {
            width: 3,
            height: 3,
            data: raw.to_vec(),
        };
        utils::graphic::save_image(&r, "output.png").unwrap();

        let texture = super::Texture::new("output.png").unwrap();
        assert_eq!(texture.get_color(0.5, 0.5), Rgb::new(0, 128, 128));
        assert_eq!(texture.get_color(0.16, 0.5), Rgb::new(0, 128, 0));
        assert_eq!(texture.get_color(0.5, 0.16), Rgb::new(0, 255, 128));
        assert_eq!(texture.get_color(0.16, 0.16), Rgb::new(0, 255, 0));
        assert_eq!(texture.get_color(0.84, 0.84), Rgb::new(0, 0, 255));
        assert_eq!(texture.get_color(0.99, 0.01), Rgb::new(0, 255, 255));
        assert_eq!(texture.get_color(0.0, 0.0), Rgb::new(0, 255, 0));
        assert_eq!(texture.get_color(1.0, 0.0), Rgb::new(0, 255, 255));
        assert_eq!(texture.get_color(0.0, 1.0), Rgb::new(0, 0, 0));
        assert_eq!(texture.get_color(1.0, 1.0), Rgb::new(0, 0, 255));
    }
}
