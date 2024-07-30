use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    pub fn new(x: i32, y: i32, z: i32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn origin() -> Point3 {
        Point3::default()
    }

    pub fn distance_to(&self, other: &Point3) -> u32 {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:}, {:}, {:})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Point3 {
    type Output = Point3;
    fn add(self, other: Point3) -> Point3 {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Point3 {
    fn add_assign(&mut self, other: Point3) {
        *self = *self + other;
    }
}

impl std::ops::Add<i32> for Point3 {
    type Output = Point3;
    fn add(self, rhs: i32) -> Point3 {
        Point3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl std::ops::AddAssign<i32> for Point3 {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Point3 {
    type Output = Point3;
    fn sub(self, other: Point3) -> Point3 {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Point3 {
    fn sub_assign(&mut self, other: Point3) {
        *self = *self - other;
    }
}

impl std::ops::Mul<i32> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: i32) -> Point3 {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::MulAssign<i32> for Point3 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for Point3 {
    type Output = Point3;
    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn airthematic() {
        let p1 = Point3::new(1, 2, 3);
        let p2 = Point3::new(4, 5, 6);
        assert_eq!(p1 + p2, Point3::new(5, 7, 9));
        assert_eq!(p1 - p2, Point3::new(-3, -3, -3));
        assert_eq!(p1 * 2, Point3::new(2, 4, 6));
        assert_eq!(p1.distance_to(&p2), 9);
    }

    #[test]
    fn negate() {
        let p1 = Point3::new(1, 2, 3);
        let p2 = Point3::new(-1, -2, -3);
        assert_eq!(-p1, p2);
    }

    #[test]
    fn test_display() {
        let p = Point3::new(1, 2, 3);
        assert_eq!(format!("{}", p), "(1, 2, 3)");
    }
}
