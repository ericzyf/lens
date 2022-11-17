pub mod vec3;
pub use vec3::Vec3;
pub mod point3;
pub use point3::Point3;
pub mod color;
pub use color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        debug_assert!(t > 0.);
        HitRecord { p, normal, t }
    }

    fn none() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: -1.,
        }
    }

    fn hitted(&self) -> bool {
        self.t > 0.
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}
