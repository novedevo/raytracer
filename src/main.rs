mod vec3;
use vec3::{RGBColour, Ray, Sphere, Vec3};

use std::io::Write;

type Colour = Vec3;
type Point = Vec3;

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1636u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_colour = r.colour(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
            let pixel_colour = RGBColour::from(pixel_colour);

            println!("{}", pixel_colour);
        }
    }
    eprintln!();
}
