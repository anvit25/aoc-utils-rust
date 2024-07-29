use crate::geom::point2d::Point;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn as_point(self) -> Point {
        match self {
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    Straight,
    Left,
    Right,
}

impl ops::Add<Turn> for Direction {
    type Output = Direction;
    fn add(self, turn: Turn) -> Direction {
        match turn {
            Turn::Straight => self,
            Turn::Left => match self {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            },
            Turn::Right => match self {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            },
        }
    }
}

impl ops::AddAssign<Turn> for Direction {
    fn add_assign(&mut self, turn: Turn) {
        *self = *self + turn;
    }
}

impl ops::Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Point;
    fn add(self, dir: Direction) -> Point {
        self + dir.as_point()
    }
}