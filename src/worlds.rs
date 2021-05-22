use rand::Rng;

use crate::{
    material::{Colour, Material},
    ray::{Camera, HittableList, Sphere},
    Point, Vec3,
};

pub fn simple_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let ground = Material::Lambertian(Colour::new(0.8, 0.8, 0.0));
    let terracotta = Material::Lambertian(Colour::new(0.7, 0.3, 0.3));
    let lapis = Material::Lambertian(Colour::new(0.1, 0.2, 0.5));
    let silver = Material::Metal(Colour::new(0.8, 0.8, 0.8));
    let gold = Material::Metal(Colour::new(0.8, 0.6, 0.2));
    let glass = Material::Dielectric(1.5);

    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, lapis));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, terracotta));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, glass));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.4, glass)); //hollow centre
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, gold));

    let origin = Point::new(3.0, 3.0, 2.0);
    let focus = Point::new(0.0, 0.0, -1.0);

    let camera = Camera::new(
        origin,
        focus,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        2.0,
        (origin - focus).length(),
    );

    (world, camera)
}

pub fn complex_random_scene(aspect_ratio: f64) -> (HittableList, Camera) {
    let mut world = HittableList::default();

    let ground = Material::Lambertian(Colour::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Point::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (centre - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Colour::random() * Colour::random();
                    world.add(Sphere::new(centre, 0.2, Material::Lambertian(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Colour::random_range(0.5, 1.0);
                    //fuzziness
                    world.add(Sphere::new(centre, 0.2, Material::Metal(albedo)))
                } else {
                    world.add(Sphere::new(centre, 0.2, Material::Dielectric(1.5)))
                }
            }
        }
    }
    world.add(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(1.5),
    ));
    world.add(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Colour::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Colour::new(0.7, 0.6, 0.5)),
    ));

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    (world, camera)
}
