use lens::*;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1280;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let ll_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // Render
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (x as f64) / ((image_width as f64) - 1.);
        let v = (y as f64) / ((image_height as f64) - 1.);

        *pixel = image::Rgb::from(ray_color(&Ray::new(
            origin,
            ll_corner + u * horizontal + v * vertical - origin,
        )));
    }

    image::imageops::flip_vertical(&imgbuf)
        .save("image.png")
        .unwrap();
    println!("Done");
}
