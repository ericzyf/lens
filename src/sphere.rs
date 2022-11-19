use super::{vec3, HitRecord, Hittable, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let hb = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = hb * hb - a * c;
        if discriminant < 0. {
            return None;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-hb - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-hb + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let front_face = vec3::dot(r.direction(), outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord::new(hit_point, normal, root, front_face))
    }
}
