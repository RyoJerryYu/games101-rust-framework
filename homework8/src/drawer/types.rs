use glam::Vec2;
use utils::triangle::Rgb;

#[derive(Clone, Copy, Debug, Default)]
pub struct XYBound<T> {
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

pub trait Object {
    fn get_bound(&self) -> XYBound<f32>;
    fn is_in_bound(&self, p: Vec2) -> bool;
    fn get_color(&self) -> &Rgb;
}