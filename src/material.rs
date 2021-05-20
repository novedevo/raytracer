use crate::ray::{HitRecord, Ray};
use crate::vec3::Vec3;

use std::fmt;

use rand::Rng;

pub type Colour = Vec3;

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
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
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

                let mut rng = rand::thread_rng();

                let direction = if cannot_refract
                    || schlick_reflectance(cos_theta, refraction_ratio) > rng.gen()
                {
                    unit_direction.reflect(rec.normal)
                } else {
                    unit_direction.refract(rec.normal, refraction_ratio)
                };

                Some((Colour::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
            }
        }
    }
}

fn schlick_reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
