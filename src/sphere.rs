use super::{vec3, HitRecord, Hittable, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let hb = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = hb * hb - a * c;
        if discriminant < 0. {
            return HitRecord::none();
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-hb - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-hb + sqrtd) / a;
            if root < t_min || root > t_max {
                return HitRecord::none();
            }
        }

        let hit_point = r.at(root);
        HitRecord::new(hit_point, (hit_point - self.center) / self.radius, root)
    }
}
