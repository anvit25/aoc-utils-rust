use std::cmp;
use std::fmt::{Debug, Display, Formatter};
use std::ops;

// A discrete point on a 2D integer grid with reversed y-axis.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn left() -> Point {
        Point::new(-1, 0)
    }

    pub fn right() -> Point {
        Point::new(1, 0)
    }

    pub fn up() -> Point {
        Point::new(0, -1)
    }

    pub fn down() -> Point {
        Point::new(0, 1)
    }

    pub fn neighbors(self) -> [Point; 4] {
        [
            self + Point::up(),
            self + Point::left(),
            self + Point::right(),
            self + Point::down(),
        ]
    }

    pub fn neighbors_diagonal(self) -> [Point; 8] {
        [
            self + Point::up(),
            self + Point::up() + Point::right(),
            self + Point::right(),
            self + Point::right() + Point::down(),
            self + Point::down(),
            self + Point::down() + Point::left(),
            self + Point::left(),
            self + Point::left() + Point::up(),
        ]
    }

    pub fn manhattan(self, other: Point) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<i32> for Point {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl cmp::PartialOrd for Point {
    fn partial_cmp(&self, rhs: &Point) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl cmp::Ord for Point {
    fn cmp(&self, rhs: &Point) -> cmp::Ordering {
        self.y.cmp(&rhs.y).then(self.x.cmp(&rhs.x))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:}, {:})", self.x, self.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Pt({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_airthemetic() {
        let mut p = Point::new(1, 2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
        assert_eq!(p + Point::new(3, 4), Point::new(4, 6));
        assert_eq!(p * 2, Point::new(2, 4));
        assert_eq!(p * -1, Point::new(-1, -2));
        assert_eq!(p.manhattan(Point::new(4, 6)), 7);

        p += 1;
        assert_eq!(p, Point::new(2, 3));

        p *= 2;
        assert_eq!(p, Point::new(4, 6));
    }

    #[test]
    fn test_point_neighbors() {
        let p = Point::new(4, 6);

        assert_eq!(
            p.neighbors(),
            [
                Point::new(4, 5),
                Point::new(3, 6),
                Point::new(5, 6),
                Point::new(4, 7),
            ]
        );

        assert_eq!(
            p.neighbors_diagonal(),
            [
                Point::new(4, 5),
                Point::new(5, 5),
                Point::new(5, 6),
                Point::new(5, 7),
                Point::new(4, 7),
                Point::new(3, 7),
                Point::new(3, 6),
                Point::new(3, 5),
            ]
        );
    }

    #[test]
    fn test_point_cmp() {
        let p = Point::new(4, 6);
        assert_eq!(p.cmp(&Point::new(4, 6)), cmp::Ordering::Equal);
        assert_eq!(p.cmp(&Point::new(4, 7)), cmp::Ordering::Less);
        assert_eq!(p.cmp(&Point::new(5, 6)), cmp::Ordering::Less);
        assert_eq!(p.cmp(&Point::new(30, 5)), cmp::Ordering::Greater);
    }
}
