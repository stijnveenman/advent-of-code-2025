use std::{collections::HashMap, marker::PhantomData};

use crate::components::Point;

use super::{char_grid::CharGrid, Grid};

#[derive(PartialEq, Eq, Clone)]
pub struct HashGrid<'a, T> {
    grid: HashMap<Point, T>,
    lower: Point,
    upper: Point,

    /// Required to specify the lifetime of a HashGrid, which is required to implement the grid
    /// trait and specify a lifetime of the return Type
    phantom: PhantomData<&'a T>,
}

impl<'a, T> HashGrid<'a, T> {
    pub fn new() -> HashGrid<'a, T> {
        HashGrid {
            grid: HashMap::new(),
            lower: Point::new(0, 0),
            upper: Point::new(0, 0),
            phantom: PhantomData,
        }
    }

    pub fn from_chargrid<F>(grid: CharGrid, convert_fn: F) -> HashGrid<'a, T>
    where
        F: Fn(char) -> Option<T>,
    {
        let (lower, upper) = grid.bounds();

        let grid = grid
            .entries()
            .filter_map(|(p, v)| convert_fn(v).map(|v| (p, v)))
            .collect();

        HashGrid {
            grid,
            lower,
            upper,
            phantom: PhantomData,
        }
    }

    pub fn with_bounds(lower: Point, upper: Point) -> HashGrid<'a, T> {
        HashGrid {
            grid: HashMap::new(),
            lower,
            upper,
            phantom: PhantomData,
        }
    }

    fn update_bounds(&mut self, point: &Point) {
        self.upper.x = self.upper.x.max(point.x);
        self.upper.y = self.upper.y.max(point.y);

        self.lower.x = self.lower.x.min(point.x);
        self.lower.y = self.lower.y.min(point.y);
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.grid.contains_key(point)
    }

    pub fn remove(&mut self, point: &Point) -> Option<T> {
        self.grid.remove(point)
    }
}

impl<'a> HashGrid<'a, char> {
    #[allow(dead_code)]
    pub fn print_char(&self) {
        self.print(|_p, c| match c {
            Some(c) => c.to_string(),
            None => " ".to_string(),
        });
    }
}

impl<'a, T> Default for HashGrid<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Grid<'a> for HashGrid<'a, T> {
    type ReturnItem = &'a T;
    type SetItem = T;

    fn bounds(&self) -> (Point, Point) {
        (self.lower, self.upper)
    }

    fn get(&'a self, point: &Point) -> Option<Self::ReturnItem> {
        self.grid.get(point)
    }

    fn set(&mut self, point: &Point, value: Self::SetItem) {
        self.update_bounds(point);

        self.grid.insert(point.to_owned(), value);
    }

    fn keys(&self) -> impl Iterator<Item = Point> {
        self.grid.keys().cloned()
    }

    fn values(&'a self) -> impl Iterator<Item = Self::ReturnItem> {
        self.grid.values()
    }

    fn entries(&'a self) -> impl Iterator<Item = (Point, Self::ReturnItem)> {
        self.grid.iter().map(|(k, v)| (k.to_owned(), v))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn bounds_should_update() {
        let mut grid = HashGrid::new();

        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(0, 0)));

        grid.set(&Point::new(1, 1), ());
        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(1, 1)));

        grid.set(&Point::new(5, 1), ());
        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(5, 1)));

        grid.set(&Point::new(2, 8), ());
        assert_eq!(grid.bounds(), (Point::new(0, 0), Point::new(5, 8)));

        grid.set(&Point::new(-2, -2), ());
        assert_eq!(grid.bounds(), (Point::new(-2, -2), Point::new(5, 8)));
    }

    #[rstest]
    fn should_keep_default_bounds() {
        let mut grid = HashGrid::with_bounds(Point::new(-5, -5), Point::new(5, 5));

        assert_eq!(grid.bounds(), (Point::new(-5, -5), Point::new(5, 5)));

        grid.set(&Point::new(1, 1), ());
        assert_eq!(grid.bounds(), (Point::new(-5, -5), Point::new(5, 5)));

        grid.set(&Point::new(10, 10), ());
        assert_eq!(grid.bounds(), (Point::new(-5, -5), Point::new(10, 10)));
    }

    #[rstest]
    fn should_set_and_get() {
        let mut grid = HashGrid::with_bounds(Point::new(-5, -5), Point::new(5, 5));

        grid.set(&Point::new(5, 5), "hello");
        assert_eq!(grid.get(&Point::new(5, 5)), Some(&"hello"));

        grid.set(&Point::new(5, 5), "world");
        assert_eq!(grid.get(&Point::new(5, 5)), Some(&"world"));

        grid.set(&Point::new(4, 5), "foo");
        assert_eq!(grid.get(&Point::new(5, 5)), Some(&"world"));
        assert_eq!(grid.get(&Point::new(4, 5)), Some(&"foo"));
    }

    #[rstest]
    fn hash_grid_can_draw() {
        let mut grid = HashGrid::new();

        for i in 0..5 {
            grid.set(&Point::new(i, i), i);
        }

        let expected = "
0....
.1...
..2..
...3.
....4
"
        .trim();

        let result = grid.draw(|_point, value| match value {
            Some(i) => i.to_string(),
            None => '.'.to_string(),
        });

        assert_eq!(result, expected)
    }
}
