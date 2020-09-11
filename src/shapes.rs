use crate::ray::*;
use crate::vec3::*;

#[derive(Copy, Clone, Debug, getset::CopyGetters, new)]
#[getset(get_copy = "pub")]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> Option<HitRecord>;
}

#[derive(Debug, getset::CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center:Point3, radius : f32) -> Self {
        assert!(radius > 0.0, "Radius of sphere is not valid: {}", radius);
        Self{
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> Option<HitRecord> {
        let oc = r.orig() - self.center;
        let a = r.dir().dot(r.dir());
        let b = 2_f32 * oc.dot(r.dir());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4_f32 * a * c;

        if discriminant <= 0_f32 {
            return None;
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 > 0.001 && t2 > 0.001 {
                let t = t1.min(t2);
                let point = r.at(t);
                let normal = (point - self.center).unit_vector();
                let hrec = HitRecord { t, point, normal };
                assert!(normal.dot(r.dir()) < 0.0);
                return Some(hrec);
            }
        }

        None
    }
}

#[derive(getset::MutGetters)]
pub struct HittableList {
    #[getset(get_mut = "pub")]
    list: Vec<Box<dyn Hittable>>,
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
        HittableList { list: Vec::new() }
    }
}
