use image::{ImageBuffer, Rgb};
use lens::*;
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

fn rand_displacement(spp: usize) -> Vec<(f64, f64)> {
    let range = Uniform::new(0., 1.);
    let mut rng = rand::thread_rng();

    let mut dp = Vec::with_capacity(spp);
    for _ in 0..spp {
        dp.push((range.sample(&mut rng), range.sample(&mut rng)));
    }
    dp
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.scatter(r) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0., 0., 0.)
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1280;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let material_ground = Material::new_lambertian(Color::new(0.8, 0.8, 0.));
    let material_center = Material::new_lambertian(Color::new(0.7, 0.3, 0.3));
    let material_left = Material::new_metal(Color::new(0.8, 0.8, 0.8));
    let material_right = Material::new_metal(Color::new(0.8, 0.6, 0.2));

    let world = Scene {
        objects: vec![
            Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground),
            Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center),
            Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left),
            Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right),
        ],
    };

    // Camera
    let cam = Camera::new();

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(image_width, image_height);
    let sample_displacement = rand_displacement(samples_per_pixel);

    // Render
    imgbuf
        .as_flat_samples_mut()
        .as_view_mut::<Rgb<u8>>()
        .unwrap()
        .image_mut_slice()
        .par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, p)| {
            let image_width = image_width as usize;
            let (x, y) = (i % image_width, i / image_width);

            let mut avg_color = Vec3::new(0., 0., 0.);
            for &(dx, dy) in &sample_displacement {
                let sx = (x as f64) + dx;
                let sy = (y as f64) + dy;
                let u = sx / ((image_width as f64) - 1.);
                let v = sy / ((image_height as f64) - 1.);
                let Color(c) = ray_color(&cam.get_ray(u, v), &world, max_depth);
                avg_color += c;
            }
            avg_color /= sample_displacement.len() as f64;

            let Rgb(pixel_color) = Rgb::from(Color(avg_color).gamma_correction());
            p.copy_from_slice(&pixel_color);
        });

    image::imageops::flip_vertical(&imgbuf)
        .save("image.png")
        .unwrap();
    println!("Done");
}
