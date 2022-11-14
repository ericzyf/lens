fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // Render
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (x as f64) / ((image_width as f64) - 1.0);
        let g = (y as f64) / ((image_height as f64) - 1.0);
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }

    image::imageops::flip_vertical(&imgbuf).save("image.png").unwrap();
    println!("Done");
}
