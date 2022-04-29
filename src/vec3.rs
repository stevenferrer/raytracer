use std::cmp::Eq;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        let x = u.e[1] * v.e[2] - u.e[2] * v.e[1];
        let y = u.e[2] * v.e[0] - u.e[0] * v.e[2];
        let z = u.e[0] * v.e[1] - u.e[1] * v.e[0];
        Vec3 { e: [x, y, z] }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn write_vector<W: Write>(mut stream: W, v: &Vec3) {
    stream
        .write_fmt(format_args!("{} {} {}", v.e[0], v.e[1], v.e[2]))
        .unwrap();
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        let [x, y, z] = self.e;
        x * x + y * y + z * z
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2]
    }
}

impl Eq for Vec3 {}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        if i > 3 {
            return &0.0;
        }

        &self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, v: Self) -> Self {
        let x = self.e[0] + v.e[0];
        let y = self.e[1] + v.e[1];
        let z = self.e[2] + v.e[2];
        Self { e: [x, y, z] }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        let x = self.x() + v.x();
        let y = self.y() + v.y();
        let z = self.z() + v.z();
        *self = Self { e: [x, y, z] };
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        (1.0 / t) * self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self *= 1.0 / t;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        let x = self.e[0] * v.e[0];
        let y = self.e[1] * v.e[1];
        let z = self.e[2] * v.e[2];
        Self::Output { e: [x, y, z] }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        let x = self.e[0] * t;
        let y = self.e[1] * t;
        let z = self.e[2] * t;
        Self::Output { e: [x, y, z] }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        let x = v.e[0] * self;
        let y = v.e[1] * self;
        let z = v.e[2] * self;
        Self::Output { e: [x, y, z] }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        let x = self.e[0] - v.e[0];
        let y = self.e[1] - v.e[1];
        let z = self.e[2] - v.e[2];
        Self::Output { e: [x, y, z] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;

    #[test]
    fn new() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let v = Vec3::new(x, y, z);
        assert_eq!(x, v.x());
        assert_eq!(y, v.y());
        assert_eq!(z, v.z());
    }

    #[test]
    fn write_vector() {
        let (x, y, z) = (1.0, 2.0, 3.0);
        let v = Vec3::new(x, y, z);

        let mut buffer = [0u8; 5];
        let writer = BufWriter::new(buffer.as_mut());
        super::write_vector(writer, &v);

        assert_eq!("1 2 3", String::from_utf8_lossy(&buffer))
    }

    #[test]
    fn length() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let v1 = Vec3::new(x1, y1, z1);

        let len = v1.length();
        assert_eq!(3.7416575, len);
    }

    #[test]
    fn length_squared() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let v1 = Vec3::new(x1, y1, z1);

        let len_squared = v1.length_squared();
        assert_eq!(14.0, len_squared);
    }

    #[test]
    fn index_op() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let v = Vec3::new(x, y, z);
        assert_eq!(x, v[0]);
        assert_eq!(y, v[1]);
        assert_eq!(z, v[2]);
    }

    #[test]
    fn negate_op() {
        let (x, y, z) = (1.0, 2.0, 3.0);

        let mut v = Vec3::new(x, y, z);
        v = -v;

        assert_eq!(-x, v.x());
        assert_eq!(-y, v.y());
        assert_eq!(-z, v.z());
    }

    #[test]
    fn add_assign_op() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (1.0, 2.0, 3.0);

        let mut v1 = Vec3::new(x1, y1, z1);
        let v2 = Vec3::new(x2, y2, z2);

        v1 += v2;

        assert_eq!(2.0, v1.x());
        assert_eq!(4.0, v1.y());
        assert_eq!(6.0, v1.z());
    }

    #[test]
    fn mul_assign_op() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let t = 3.0;

        let mut v1 = Vec3::new(x1, y1, z1);

        v1 *= t;

        assert_eq!(3.0, v1.x());
        assert_eq!(6.0, v1.y());
        assert_eq!(9.0, v1.z());
    }

    #[test]
    fn div_assign_op() {
        let (x1, y1, z1) = (3.0, 6.0, 9.0);
        let t = 3.0;

        let mut v1 = Vec3::new(x1, y1, z1);

        v1 /= t;

        assert_eq!(1.0, v1.x());
        assert_eq!(2.0, v1.y());
        assert_eq!(3.0, v1.z());
    }

    #[test]
    fn mul_op() {
        let (x1, y1, z1) = (1.0, 2.0, 3.0);
        let (x2, y2, z2) = (1.0, 2.0, 3.0);

        let v1 = Vec3::new(x1, y1, z1);
        let v2 = Vec3::new(x2, y2, z2);

        // v * v
        let v3 = v1 * v2;

        assert_eq!(1.0, v3.x());
        assert_eq!(4.0, v3.y());
        assert_eq!(9.0, v3.z());

        // v * t or t * v
        let v4 = 2.0 * v3 * 1.0;
        assert_eq!(2.0, v4.x());
        assert_eq!(8.0, v4.y());
        assert_eq!(18.0, v4.z());
    }

    #[test]
    fn eq_op() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }
}
