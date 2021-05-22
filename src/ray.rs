use crate::material::{Colour, Material};
use crate::vec3::{Point, Vec3};

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
}

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    t: f64,
    material: Material,
    pub front_face: bool,
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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}
impl Camera {
    pub fn new(
        origin: Point,
        focus: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (origin - focus).unit();
        let u = Vec3::cross(vup, w);
        let v = Vec3::cross(w, u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.e[0] + self.v * rd.e[1];

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
