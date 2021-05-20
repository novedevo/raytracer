use rand::{self, Rng};
use std::{
    fmt,
    ops::{Add, Div, Index, Mul, Neg, Sub},
};

type Colour = Vec3;
type Point = Vec3;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }
    fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
    }
    fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            e: [
                lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
                lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
                lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
            ],
        }
    }
    fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    fn unit(self) -> Self {
        self / self.length()
    }
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
    fn random_range(low: f64, high: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(low..high),
            rng.gen_range(low..high),
            rng.gen_range(low..high),
        )
    }
    fn random_unit() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p.unit();
        }
    }
    fn is_near_zero(self) -> bool {
        self.e.iter().all(|elem| elem.abs() < 1e-8)
    }
    fn reflect(self, normal: Self) -> Self {
        self - 2.0 * Self::dot(self, normal) * normal
    }
    fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = Self::dot(normal, -self).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct RGBColour {
    r: u8,
    g: u8,
    b: u8,
}
impl fmt::Display for RGBColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
impl From<Vec3> for RGBColour {
    fn from(other: Vec3) -> Self {
        Self {
            g: (other.e[1].sqrt() * 255.999) as u8,
            b: (other.e[2].sqrt() * 255.999) as u8,
            r: (other.e[0].sqrt() * 255.999) as u8,
        }
    }
}
impl From<&RGBColour> for [u8; 3] {
    fn from(colour: &RGBColour) -> Self {
        [colour.r, colour.g, colour.b]
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
    pub fn colour(self, world: &HittableList, max_depth: usize) -> Colour {
        if max_depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(self, 0.00001, f64::INFINITY) {
            if let Some((attentuation, scattered)) = rec.material.scatter(self, &rec) {
                return attentuation * scattered.colour(world, max_depth - 1);
            }
            return Colour::new(0.0, 0.0, 0.0);
        }
        let t = 0.5 * (self.direction.unit().e[1] + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
    // fn hit_sphere(centre: Point, radius: f64, r: Self) -> Option<f64> {
    //     let oc = r.origin - centre;

    //     //compute quadratic equation coefficients
    //     let a = r.direction.length_squared();
    //     let half_b = Vec3::dot(oc, r.direction);
    //     let c = oc.length_squared() - radius * radius;
    //     let discriminant = half_b.powi(2) - a * c;
    //     if discriminant < 0.0 {
    //         None
    //     } else {
    //         Some((-half_b - discriminant.sqrt()) / a) //quadratic formula
    //     }
    // }
}

// #[derive(Clone, Copy)]
// pub enum Normal {
//     FrontfaceNormal(Vec3),
//     BackfaceNormal(Vec3),
// }
// impl Default for Normal {
//     fn default() -> Self {
//         Self::FrontfaceNormal(Vec3::default())
//     }
// }

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
    material: Material,
    front_face: bool,
}
impl HitRecord {
    fn front_face(r: Ray, normal: Vec3) -> bool {
        Vec3::dot(r.direction, normal) < 0.0
    }
}

trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    centre: Point,
    radius: f64,
    material: Material,
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.centre;

        //compute quadratic equation coefficients
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let normal = (r.at(root) - self.centre) / self.radius;
        let front_face = HitRecord::front_face(r, normal);

        Some(HitRecord {
            t: root,
            p: r.at(root),
            normal: if front_face { normal } else { -normal },
            material: self.material,
            front_face,
        })
    }
}
impl Sphere {
    pub fn new(centre: Point, radius: f64, material: Material) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Sphere>,
}
impl HittableList {
    pub fn add(&mut self, new: Sphere) {
        self.objects.push(new)
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest = HitRecord {
            t: t_max,
            ..Default::default()
        };

        for object in &self.objects {
            closest = if let Some(closest) = object.hit(r, t_min, closest.t) {
                hit_anything = true;
                closest
            } else {
                closest
            };
        }

        if hit_anything {
            Some(closest)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new() -> Self {
        //Image
        let aspect_ratio = 16.0 / 9.0;

        //Camera
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let tilt_radians = 2.0 * std::f64::consts::PI;

        let horizontal = Vec3::new(
            viewport_width * tilt_radians.cos(),
            viewport_width * tilt_radians.sin(),
            0.0,
        );
        let vertical = Vec3::new(
            viewport_height * -tilt_radians.sin(),
            viewport_height * tilt_radians.cos(),
            0.0,
        );

        Self {
            origin: Point::default(),
            horizontal,
            vertical,
            lower_left_corner: Point::default()
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Colour),
    Metal(Colour),
    Dielectric(f64),
}
impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Colour::new(0.5, 0.5, 0.5))
    }
}

impl Material {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Self::Lambertian(albedo) => {
                let scatter_direction = rec.normal + Vec3::random_unit();
                Some((
                    *albedo,
                    Ray::new(
                        rec.p,
                        match scatter_direction.is_near_zero() {
                            true => rec.normal,
                            false => scatter_direction,
                        },
                    ),
                ))
            }
            Self::Metal(albedo) => {
                let reflected = r_in.direction.reflect(rec.normal).unit();
                let scattered = Ray::new(rec.p, reflected);
                match Vec3::dot(scattered.direction, rec.normal) > 0.0 {
                    false => None,
                    true => Some((*albedo, scattered)),
                }
            }
            Self::Dielectric(ir) => {
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { *ir };

                let unit_direction = r_in.direction.unit();

                let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract {
                    unit_direction.reflect(rec.normal)
                } else {
                    unit_direction.refract(rec.normal, refraction_ratio)
                };
                
                Some((Colour::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
            }
        }
    }
}
