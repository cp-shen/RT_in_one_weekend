//#![allow(dead_code)]
use crate::vec3;

pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> vec3::Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {}
