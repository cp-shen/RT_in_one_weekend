use crate::ray::*;
use crate::vec3::*;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.dot(r.dir);
        let b = 2_f32 * oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4_f32 * a * c;

        if discriminant < 0_f32 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 > 0.0 && t2 > 0.0 {
                let t = t1.min(t2);
                let point = r.at(t);
                let normal = point - self.center;
                let hrec = HitRecord { t, point, normal };
                return Some(hrec);
            }
        }

        None
    }
}

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray) -> Option<HitRecord> {
        let mut t = f32::MAX;
        let mut ret: Option<HitRecord> = None;

        self.list.iter().for_each(|hittable| {
            let hrec_op = hittable.hit(r);
            if let Some(hrec) = hrec_op {
                if hrec.t < t {
                    t = hrec.t;
                    ret = hrec_op;
                }
            }
        });

        ret
    }
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            list : Vec::new()
        }
    }
}
