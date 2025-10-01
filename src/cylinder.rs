use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Cylinder {
        Cylinder { center, radius, height, material }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t_body = f64::INFINITY;
        let mut t_caps = f64::INFINITY;

        // Body intersection
        let oc = r.origin - self.center;
        let a = r.direction.x * r.direction.x + r.direction.z * r.direction.z;
        let b = 2.0 * (oc.x * r.direction.x + oc.z * r.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let y0 = r.origin.y + t0 * r.direction.y;
            let y1 = r.origin.y + t1 * r.direction.y;

            if y0 > self.center.y && y0 < self.center.y + self.height && t0 > t_min && t0 < t_max {
                t_body = t0;
            }
            if y1 > self.center.y && y1 < self.center.y + self.height && t1 > t_min && t1 < t_max && t1 < t_body {
                t_body = t1;
            }
        }

        // Caps intersection
        // Top cap
        let t_top = (self.center.y + self.height - r.origin.y) / r.direction.y;
        if t_top > t_min && t_top < t_max {
            let p = r.at(t_top);
            if (p.x - self.center.x).powi(2) + (p.z - self.center.z).powi(2) <= self.radius.powi(2) {
                t_caps = t_top;
            }
        }

        // Bottom cap
        let t_bottom = (self.center.y - r.origin.y) / r.direction.y;
        if t_bottom > t_min && t_bottom < t_max && t_bottom < t_caps {
            let p = r.at(t_bottom);
            if (p.x - self.center.x).powi(2) + (p.z - self.center.z).powi(2) <= self.radius.powi(2) {
                t_caps = t_bottom;
            }
        }

        let t = if t_body < t_caps { t_body } else { t_caps };

        if t.is_infinite() {
            return None;
        }

        let p = r.at(t);
        let normal = self.get_normal(p, t == t_caps);

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

impl Cylinder {
    fn get_normal(&self, p: Vec3, is_caps: bool) -> Vec3 {
        if is_caps {
            if (p.y - (self.center.y + self.height)).abs() < 1e-6 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(0.0, -1.0, 0.0)
            }
        } else {
            Vec3::new(p.x - self.center.x, 0.0, p.z - self.center.z).unit_vector()
        }
    }
}
