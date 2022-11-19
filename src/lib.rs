use rand::distributions::{Distribution, Uniform};

pub mod vec3;
pub use vec3::Vec3;
pub mod point3;
pub use point3::Point3;
pub mod color;
pub use color::Color;
pub mod sphere;
pub use sphere::Sphere;
pub mod scene;
pub use scene::Scene;
pub mod camera;
pub use camera::Camera;
pub mod material;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
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
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        debug_assert!(t > 0.);
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn hit_point(&self) -> Point3 {
        debug_assert!(self.hitted());
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        debug_assert!(self.hitted());
        self.normal
    }

    pub fn t(&self) -> f64 {
        debug_assert!(self.hitted());
        self.t
    }

    pub fn none() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: -1.,
            front_face: false,
        }
    }

    pub fn hitted(&self) -> bool {
        self.t > 0.
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}

pub fn random_in_unit_sphere() -> Vec3 {
    let range = Uniform::new_inclusive(-1., 1.);
    let mut rng = rand::thread_rng();

    loop {
        let v = Vec3::new(
            range.sample(&mut rng),
            range.sample(&mut rng),
            range.sample(&mut rng),
        );
        if v.length_squared() < 1. {
            break v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}
