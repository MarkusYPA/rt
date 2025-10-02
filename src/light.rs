use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub brightness: f64,
}

impl Light {
    pub fn new(position: Vec3, brightness: f64) -> Self {
        Light {
            position,
            brightness,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            position: Vec3::new(10.0, 14.0, 10.0),
            brightness: 1.0,
        }
    }
}
