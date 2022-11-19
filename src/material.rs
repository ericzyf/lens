use super::{Color, HitRecord, Ray};

pub trait Scatterable {
    fn scatter(r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian,
}
