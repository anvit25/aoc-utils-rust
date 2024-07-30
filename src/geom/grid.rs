use super::point2d::Point;
use std::ops;

struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, default: T) -> Self {
        Grid {
            data: vec![vec![default; width]; height],
            width,
            height,
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    fn new_default(width: usize, height: usize) -> Self {
        Grid::new(width, height, T::default())
    }
}

impl<T> ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.data[y][x]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[y][x]
    }
}

impl<T> ops::Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, point: Point) -> &T {
        self.index((point.y as usize, point.x as usize))
    }
}

impl<T> ops::IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, point: Point) -> &mut T {
        self.index_mut((point.y as usize, point.x as usize))
    }
}

impl<T> Grid<T> {
    pub fn in_bounds(&self, point: Point) -> bool {
        let x = point.x;
        let y = point.y;
        if x < 0 || y < 0 {
            return false;
        }
        let x = point.x as usize;
        let y = point.y as usize;
        x < self.width && y < self.height
    }

    pub fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = point.neighbors().to_vec();
        neighbors.retain(|&p| self.in_bounds(p));
        neighbors
    }

    pub fn neighbors_diagonal(&self, point: Point) -> Vec<Point> {
        let mut neighbors = point.neighbors_diagonal().to_vec();
        neighbors.retain(|&p| self.in_bounds(p));
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let mut grid = Grid::new(3, 3, 0);
        grid[(1, 2)] = 1;
        assert_eq!(grid[(1, 2)], 1);
        assert_eq!(grid[Point::new(2, 1)], 1);

        let to_check = &mut grid[Point::new(1, 1)];
        *to_check = 2;

        assert_eq!(grid[(1, 1)], 2);
    }
}
