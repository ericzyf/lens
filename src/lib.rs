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
