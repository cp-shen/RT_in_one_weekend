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

    // pub fn hit_sphere(
    //     &self,
    //     center: vec3::Point3,
    //     radius: f32,
    // ) -> Option<vec3::Point3> {
    //     let r = &self;
    //     let oc = r.orig - center;
    //     let a = r.dir.dot(r.dir);
    //     let b = 2_f32 * oc.dot(r.dir);
    //     let c = oc.dot(oc) - radius * radius;
    //     let discriminant = b * b - 4_f32 * a * c;
    // 
    //     if discriminant < 0_f32 {
    //         return None;
    //     } else {
    //         let t = (-b - discriminant.sqrt()) / (2.0 * a);
    //         if t > 0.0 {
    //             return Some(r.at(t));
    //         }
    //     }
    // 
    //     None
    // }
}

#[cfg(test)]
mod tests {}
