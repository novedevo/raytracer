mod vec3;
use rand::{self, Rng};
use vec3::{Camera, HittableList, Material, RGBColour, Sphere, Vec3};

use std::io::BufWriter;
use std::thread;
use std::{fs::File, io::Write, path::Path}; //to flush the print! call after each scanline updates

use png::Encoder;

type Colour = Vec3;
type Point = Vec3;

//Image parameters
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;
const NUM_THREADS: usize = 12;

fn main() {
    //Worldgen!
    let mut world = HittableList::default();

    let ground = Material::Lambertian(Colour::new(0.8, 0.8, 0.0));
    let terracotta = Material::Lambertian(Colour::new(0.7, 0.3, 0.3));
    let silver = Material::Metal(Colour::new(0.8, 0.8, 0.8));
    let gold = Material::Metal(Colour::new(0.8, 0.6, 0.2));
    let glass = Material::Dielectric(1.5);

    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, terracotta));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, glass));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.4, glass)); //hollow centre
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, gold));

    let camera = Camera::new(
        50.0,
        ASPECT_RATIO,
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut png_encoder = Encoder::new(
        BufWriter::new(File::create(Path::new("out.png")).unwrap()),
        IMAGE_WIDTH as u32,
        IMAGE_HEIGHT as u32,
    );
    png_encoder.set_color(png::ColorType::RGB);

    let mut png_writer = png_encoder.write_header().unwrap();

    //Render
    let mut threads = vec![];

    for thread_num in 0..NUM_THREADS {
        //clone the world so that we can move it into the closure
        let world = world.clone();

        threads.push(thread::spawn(move || render(&camera, thread_num, &world)));
    }

    let mut component_vec = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT * 3];

    //wait for all threads to finish execution, then fill the component vector
    for handle in threads {
        for (colours, row) in handle.join().unwrap() {
            for (row_index, pixel) in colours.iter().enumerate() {
                let components: [u8; 3] = pixel.into();
                let index = (IMAGE_WIDTH * (IMAGE_HEIGHT - 1 - row) + row_index) * 3;

                component_vec[index] = components[0];
                component_vec[index + 1] = components[1];
                component_vec[index + 2] = components[2];
            }
        }
    }

    println!("\rScanlines remaining: 0");

    png_writer
        .write_image_data(&component_vec)
        .expect("Failed to write PNG data");
}

fn render(
    camera: &Camera,
    thread_num: usize,
    world: &HittableList,
) -> Vec<([RGBColour; IMAGE_WIDTH], usize)> {
    let mut rng = rand::thread_rng();
    let mut lines = vec![];
    for j in (0..IMAGE_HEIGHT).rev() {
        if j % NUM_THREADS != thread_num {
            continue;
        }

        //update progress indicator
        print!("\rScanlines remaining: {} ", j);
        std::io::stdout().flush().unwrap();

        let mut scanline = [RGBColour::default(); IMAGE_WIDTH];

        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_colour = pixel_colour + r.colour(world, MAX_DEPTH);
            }
            scanline[i] = RGBColour::from(pixel_colour / SAMPLES_PER_PIXEL as f64);
        }
        lines.push((scanline, j));
    }
    lines
}
