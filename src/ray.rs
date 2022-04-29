use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(5.0, 3.0, 4.0);
        let r = Ray::new(p, v);

        assert_eq!(p, r.origin());
        assert_eq!(v, r.direction());
    }

    #[test]
    fn at() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(5.0, 3.0, 4.0);
        let r = Ray::new(p, v);

        let p_at = Point3::new(11.0, 8.0, 11.0);
        assert_eq!(p_at, r.at(2.0));
    }
}
