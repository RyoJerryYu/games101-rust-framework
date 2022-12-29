use obj::load_obj;
use utils::triangle::{Triangle, Rgb};
use anyhow::Result;

fn main() -> Result<()> {
    let input = std::io::BufReader::new(std::fs::File::open("input.obj")?);
    let model: obj::Obj<obj::TexturedVertex, u8> = load_obj(input)?;
    model.vertices.iter().for_each(|v| println!("{:?}", v));
    return  Ok(());
}
