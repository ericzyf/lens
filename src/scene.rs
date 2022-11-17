use super::{HitRecord, Hittable, Ray, Sphere};

pub struct Scene {
    pub objects: Vec<Sphere>,
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> HitRecord {
        let mut closest_hit = HitRecord::none();

        for obj in &self.objects {
            let hit_rec = obj.hit(r, t_min, t_max);
            if hit_rec.hitted() {
                t_max = hit_rec.t();
                closest_hit = hit_rec;
            }
        }

        closest_hit
    }
}
