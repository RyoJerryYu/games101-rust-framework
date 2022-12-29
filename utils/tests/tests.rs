#[cfg(test)]
mod tests {
    use utils::triangle::{Triangle, Rgb};
    use glam::Vec3;
    #[test]
    fn triangle_test() {
        let vertex = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let colors = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let mut t = Triangle::new();
        for vert_ind in 0..3 {
            t.set_vertex(vert_ind, Vec3::from_array(vertex[vert_ind]));
            t.set_color(vert_ind, Rgb::from(&Vec3::from_array(colors[vert_ind])));
        }

        assert!(t.a() == Vec3::from_array(vertex[0]));
        assert!(t.b() == Vec3::from_array(vertex[1]));
        assert!(t.c() == Vec3::from_array(vertex[2]));

        let color = t.get_color();
        assert!(color == Rgb::from(&Vec3::from_array(colors[0])));
    }
}