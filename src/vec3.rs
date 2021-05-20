use rand::{self, Rng};
use std::{
    fmt,
    ops::{Add, Div, Index, Mul, Neg, Sub},
};

type Colour = Vec3;
type Point = Vec3;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
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
// impl Mul for Vec3 {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self {
//         Self {
//             e: [
//                 self.e[0] * other.e[0],
//                 self.e[1] * other.e[1],
//                 self.e[2] * other.e[2],
//             ],
//         }
//     }
// }
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
    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
    }
    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self {
            e: [
                lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
                lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
                lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
            ],
        }
    }
    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit(self) -> Self {
        self / self.length()
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
    pub fn random_range(low: f64, high: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(low..high),
            rng.gen_range(low..high),
            rng.gen_range(low..high),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p.unit();
        }
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
    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
    pub fn colour(self, world: &dyn Hittable, max_depth: usize) -> Colour {
        if max_depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(self, 0.00001, f64::INFINITY) {
            let target = rec.p
                + match rec.normal {
                    Normal::FrontfaceNormal(normal) => normal,
                    Normal::BackfaceNormal(normal) => normal,
                }
                + Vec3::random_in_unit_sphere();
            return 0.5 * Self::new(rec.p, target - rec.p).colour(world, max_depth - 1);
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

#[derive(Clone, Copy)]
pub enum Normal {
    FrontfaceNormal(Vec3),
    BackfaceNormal(Vec3),
}
impl Default for Normal {
    fn default() -> Self {
        Self::FrontfaceNormal(Vec3::default())
    }
}

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    p: Point,
    normal: Normal,
    t: f64,
}
impl HitRecord {
    fn normalize(r: Ray, normal: Vec3) -> Normal {
        if Vec3::dot(r.direction, normal) < 0.0 {
            Normal::FrontfaceNormal(normal)
        } else {
            Normal::BackfaceNormal(-normal)
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    centre: Point,
    radius: f64,
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

        Some(HitRecord {
            t: root,
            p: r.at(root),
            normal: HitRecord::normalize(r, (r.at(root) - self.centre) / self.radius),
        })
    }
}
impl Sphere {
    pub fn new(centre: Point, radius: f64) -> Self {
        Self { centre, radius }
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

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

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
