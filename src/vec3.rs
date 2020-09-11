use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub type Color = Vec3;
pub type Point3 = Vec3;

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// impl ops::AddAssign<Vec3> for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self + -rhs
    }
}

// impl ops::SubAssign<Vec3> for Vec3 {
//     fn sub_assign(&mut self, rhs: Vec3) {
//         *self = *self - rhs;
//     }
// }

// impl ops::MulAssign<f32> for Vec3 {
//     fn mul_assign(&mut self, rhs: f32) {
//         *self = *self * rhs;
//     }
// }

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length_sqared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_sqared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn gamma_correct(&self, gamma: u32) -> Self {
        let pow = 1.0 / gamma as f32;
        Color::new(self.0.powf(pow), self.1.powf(pow), self.2.powf(pow))
    }

    pub fn unit_vector(&self) -> Vec3 {
        assert!(self.length() > 0_f32, "Vector length is zero!");

        *self * (1_f32 / self.length())
    }

    pub fn to_8bit_color(&self) -> (u8, u8, u8) {
        let r = self.0.min(1_f32).max(0_f32) * 255_f32;
        let g = self.1.min(1_f32).max(0_f32) * 255_f32;
        let b = self.2.min(1_f32).max(0_f32) * 255_f32;
        (r.round() as u8, g.round() as u8, b.round() as u8)
    }

    pub fn lerp(a: Self, b: Self, t: f32) -> Vec3 {
        let t_clamped = t.min(1_f32).max(0_f32);
        a * (1_f32 - t_clamped) + b * t_clamped
    }
}
