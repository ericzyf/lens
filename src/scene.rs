use super::{HitRecord, Hittable, Ray, Sphere};

pub struct Scene {
    pub objects: Vec<Sphere>,
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, t_max) {
                t_max = rec.t();
                closest_hit = Some(rec);
            }
        }

        closest_hit
    }
}
