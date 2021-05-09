use std::{
    fmt,
    ops::{Add, Div, Index, Mul, Sub},
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
            r: (other.e[0] * 255.999) as u8,
            g: (other.e[1] * 255.999) as u8,
            b: (other.e[2] * 255.999) as u8,
        }
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
    pub fn colour(self) -> Colour {
        if let Some(t) = Self::hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, self) {
            let N = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return 0.5 * Colour::new(N[0] + 1.0, N[1] + 1.0, N[2] + 1.0);
        }
        let t = 0.5 * (self.direction.unit().e[1] + 1.0);
        (1.0 - t) * Colour::new(0.0, 0.0, 0.0) + t * Colour::new(0.0, 0.2, 1.0)
    }
    fn hit_sphere(centre: Point, radius: f64, r: Self) -> Option<f64> {
        let oc = r.origin - centre;

        //compute quadratic equation coefficients
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a) //quadratic formula
        }
    }
}

pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
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
            normal: (r.at(root) - self.centre) / self.radius,
        })
    }
}
impl Sphere {
    fn new(centre: Point, radius: f64) -> Self {
        Self { centre, radius }
    }
}
