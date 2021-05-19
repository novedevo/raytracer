mod vec3;
use rand::{self, Rng};
use vec3::{Camera, HittableList, RGBColour, Sphere, Vec3};

use std::io::BufWriter;
use std::{fs::File, io::Write, path::Path}; //to flush the print! call after each scanline updates

type Colour = Vec3;
type Point = Vec3;

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920 / 4;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point::new(0.5, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut out_file = BufWriter::new(File::create(Path::new("out.ppm")).unwrap());

    //Render
    write!(out_file, "P3\n{} {}\n255\n", image_width, image_height)
        .expect("could not write to file!");
    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_colour = pixel_colour + r.colour(&world, max_depth);
            }
            let pixel_colour = RGBColour::from(pixel_colour / samples_per_pixel as f64);
            writeln!(out_file, "{}", pixel_colour).unwrap();
        }
    }
    eprintln!();

    out_file.flush().unwrap();
}
