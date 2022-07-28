#[cfg(test)]
mod tests {
    use glam::Vec3;
    use homework2::{inside_triangle, triangle::Triangle};

    #[test]
    fn your_test() {
        let a = Vec3::new(3.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 4.0, 0.0);
        let c = Vec3::new(0.0, 0.0, 0.0);
        let mut abc = Triangle::new();
        abc.set_vertex(0, a);
        abc.set_vertex(1, b);
        abc.set_vertex(2, c);

        assert_eq!(inside_triangle(1.0, 1.0, &abc), true);
        assert_eq!(inside_triangle(0.0, 0.0, &abc), true);
        assert_eq!(inside_triangle(-1.0, 1.0, &abc), false);
        assert_eq!(inside_triangle(1.0, -1.0, &abc), false);
        assert_eq!(inside_triangle(4.0, 3.0, &abc), false);
    }
}
