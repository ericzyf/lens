use super::{vec3, Color, HitRecord, Ray};

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian(Lambertian { albedo })
    }

    pub fn new_metal(albedo: Color) -> Self {
        Self::Metal(Metal { albedo })
    }
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Self::Lambertian(ref l) => l.scatter(r_in, rec),
            Self::Metal(ref m) => m.scatter(r_in, rec),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal() + super::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        Some((self.albedo, Ray::new(rec.hit_point(), scatter_direction)))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color,
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vec3::reflect(r_in.direction().normalize(), rec.normal());
        if vec3::dot(reflected, rec.normal()) > 0. {
            Some((self.albedo, Ray::new(rec.hit_point(), reflected)))
        } else {
            None
        }
    }
}
