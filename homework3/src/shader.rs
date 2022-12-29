use anyhow::Result;
use glam::{Vec2, Vec3};

/*
class Texture{
private:
    cv::Mat image_data;

public:
    Texture(const std::string& name)
    {
        image_data = cv::imread(name);
        cv::cvtColor(image_data, image_data, cv::COLOR_RGB2BGR);
        width = image_data.cols;
        height = image_data.rows;
    }

    int width, height;

    Eigen::Vector3f getColor(float u, float v)
    {
        auto u_img = u * width;
        auto v_img = (1 - v) * height;
        auto color = image_data.at<cv::Vec3b>(v_img, u_img);
        return Eigen::Vector3f(color[0], color[1], color[2]);
    }

};
 */

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

    pub fn get_color(&self, u: f32, v: f32) -> utils::triangle::Rgb {
        let u_img = u * self.width as f32;
        let v_img = (1.0 - v) * self.height as f32;
        let color = self.img.get_pixel(u_img as u32, v_img as u32);
        utils::triangle::Rgb::from(color)
    }
}

pub struct FragmentShaderPayload {
    pub view_pos: Vec3,
    pub color: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
    pub texture: Option<Texture>,
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
}
