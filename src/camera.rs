use super::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    ll_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let ll_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        Camera {
            origin,
            ll_corner,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.ll_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
