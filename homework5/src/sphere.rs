use glam::Vec3;

use crate::object::{MaterialType, Object, ObjectRenderPayload};

// return x0 < x1
#[inline]
fn solve_quadratic(a: f32, b: f32, c: f32, x0: &mut f32, x1: &mut f32) -> bool {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return false;
    }
    if discr == 0.0 {
        *x0 = -0.5 * b / a;
        *x1 = *x0;
    } else {
        let q = if b > 0.0 {
            -0.5 * (b + discr.sqrt())
        } else {
            -0.5 * (b - discr.sqrt())
        };
        *x0 = q / a;
        *x1 = c / q;
    }

    if x0 > x1 {
        (*x0, *x1) = (*x1, *x0);
    }

    return true;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    pub render_payload: ObjectRenderPayload,
}

impl Object for Sphere {
    // intersect only when (p - center) ^ 2 = radius ^ 2
    // where p = orig + t * dir
    // => (orig - center)^2 - radius^2 - 2*dir.dot(orig-center)t + dir^2*t^2 = 0
    // => a = dir^2
    //    b = 2*dir.dot(orig-center)
    //    c = (orig-center)^2 - radius^2
    fn intersect(
        &self,
        orig: &glam::Vec3,
        dir: &glam::Vec3,
        tnear: &mut f32,
        index: &mut usize,
        uv: &mut glam::Vec2,
    ) -> bool {
        // analytic solution
        let l = *orig - self.center;
        let a = dir.dot(*dir);
        let b = 2.0 * dir.dot(l);
        let c = l.dot(l) - self.radius * self.radius;

        let (mut t0, mut t1) = (0.0, 0.0);
        if !solve_quadratic(a, b, c, &mut t0, &mut t1) {
            return false;
        }
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return false;
        }

        *tnear = t0;
        return true;
    }

    fn get_surface_properties(
        &self,
        p: &glam::Vec3,
        pp: &glam::Vec3,
        index: &usize,
        uv: &glam::Vec2,
        n: &mut glam::Vec3,
        st: &mut glam::Vec2,
    ) {
        *n = (*p - self.center).normalize();
    }

    fn get_render_payload(&self) -> &ObjectRenderPayload {
        &self.render_payload
    }
}

impl Sphere {
    pub fn new(c: &Vec3, r: f32) -> Self {
        Self {
            center: *c,
            radius: r,
            render_payload: ObjectRenderPayload::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec2;

    use super::*;

    fn assert_f32_eq(a: f32, b: f32) {
        assert!((a - b).abs() < 0.00001, "a={} b={}", a, b)
    }

    #[test]
    fn test_solve_quadratic() {
        struct TestCase {
            abc: (f32, f32, f32),
            want: (f32, f32),
        }

        impl TestCase {
            fn a(&self) -> f32 {
                self.abc.0
            }
            fn b(&self) -> f32 {
                self.abc.1
            }
            fn c(&self) -> f32 {
                self.abc.2
            }
        }

        let test_cases = [
            TestCase {
                abc: (1.0, 0.0, -1.0),
                want: (-1.0, 1.0),
            },
            TestCase {
                abc: (1.0, 2.0, 1.0),
                want: (-1.0, -1.0),
            },
            TestCase {
                abc: (1.0, -2.0, 1.0),
                want: (1.0, 1.0),
            },
            TestCase {
                abc: (-1.0, 2.0, -1.0),
                want: (1.0, 1.0),
            },
            TestCase {
                abc: (1.0, -5.0, 6.0),
                want: (2.0, 3.0),
            },
        ];

        for tc in test_cases {
            let (mut x0, mut x1) = (0.0, 0.0);
            let ok = solve_quadratic(tc.a(), tc.b(), tc.c(), &mut x0, &mut x1);
            assert!(ok);
            assert_f32_eq(tc.want.0, x0);
            assert_f32_eq(tc.want.1, x1);
        }
    }

    #[test]
    fn test_intersect() {
        let sphere = Sphere::new(&Vec3::ONE, 1.0);

        struct TestCase {
            orig: Vec3,
            dir: Vec3,
            is_intersect: bool,
            tnear: f32,
        }

        let cases = [
            TestCase {
                orig: Vec3::ZERO,
                dir: Vec3::ONE,
                is_intersect: true,
                tnear: (1.0 - 3.0f32.sqrt().recip()),
            },
            TestCase {
                orig: Vec3::ZERO,
                dir: -Vec3::ONE,
                is_intersect: false,
                tnear: 0.0,
            },
            TestCase {
                orig: Vec3 { x: -1.0, y: 1.0, z: 1.0 },
                dir: Vec3::X,
                is_intersect: true,
                tnear: 1.0,
            }
        ];

        for tc in cases {
            let mut tnear = 0.0;
            let (mut index, mut uv) = (0, Vec2::ZERO);
            let is_intersect = sphere.intersect(&tc.orig, &tc.dir, &mut tnear, &mut index, &mut uv);
            assert_eq!(is_intersect, tc.is_intersect);
            if tc.is_intersect {
                assert_f32_eq(tc.tnear, tnear);
            }
        }
    }
}
