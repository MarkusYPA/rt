use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Cube {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmin = (self.min.x - r.origin.x) / r.direction.x;
        let mut tmax = (self.max.x - r.origin.x) / r.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - r.origin.y) / r.direction.y;
        let mut tymax = (self.max.y - r.origin.y) / r.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - r.origin.z) / r.direction.z;
        let mut tzmax = (self.max.z - r.origin.z) / r.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        let t = if tmin > t_min { tmin } else { tmax };

        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        let normal = self.get_normal(p);

        let mut rec = HitRecord {
            t,
            p,
            normal,
            front_face: false, // Placeholder
            material: &self.material,
        };
        rec.set_face_normal(r, normal);
        Some(rec)
    }
}

impl Cube {
    fn get_normal(&self, p: Vec3) -> Vec3 {
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        if (p.x - self.min.x).abs() < 1e-6 {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (p.x - self.max.x).abs() < 1e-6 {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (p.y - self.min.y).abs() < 1e-6 {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (p.y - self.max.y).abs() < 1e-6 {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (p.z - self.min.z).abs() < 1e-6 {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (p.z - self.max.z).abs() < 1e-6 {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }
        normal
    }
}
