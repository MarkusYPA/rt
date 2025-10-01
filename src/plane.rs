use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use crate::material::Material;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane { point, normal: normal.unit_vector(), material }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denominator = self.normal.dot(r.direction);
        if denominator.abs() > 1e-6 {
            let v = self.point - r.origin;
            let t = v.dot(self.normal) / denominator;
            if t >= t_min && t <= t_max {
                let mut rec = HitRecord {
                    t,
                    p: r.at(t),
                    normal: self.normal,
                    front_face: false, // Placeholder
                    material: &self.material,
                };
                rec.set_face_normal(r, self.normal);
                return Some(rec);
            }
        }
        None
    }
}
