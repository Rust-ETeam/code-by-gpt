use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl<'a, 'b> Add<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn add(self, other: &'b Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, 'b> Sub<&'b Vector3> for &'a Vector3 {
    type Output = Vector3;

    fn sub(self, other: &'b Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}