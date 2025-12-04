use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const UP: Point = Point { x: 0, y: -1 };

    pub const UP_RIGHT: Point = Point { x: 1, y: -1 };
    pub const UP_LEFT: Point = Point { x: -1, y: -1 };
    pub const DOWN_RIGHT: Point = Point { x: 1, y: 1 };
    pub const DOWN_LEFT: Point = Point { x: -1, y: 1 };

    pub const DIRECTIONS_4: [Point; 4] = [Self::RIGHT, Self::LEFT, Self::DOWN, Self::UP];

    pub const DIRECTIONS_8: [Point; 8] = [
        Self::RIGHT,
        Self::LEFT,
        Self::DOWN,
        Self::UP,
        Self::UP_RIGHT,
        Self::UP_LEFT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
    ];

    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    /// Parses a Point neperated by a seperator
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::parse_seperated("4,-5", ",").unwrap(), Point::new(4, -5));
    /// ```
    pub fn parse_seperated(input: &str, seperator: &str) -> Result<Point, String> {
        let Some((x, y)) = input.split_once(seperator) else {
            return Err(format!("{}: seperator [{}] not found", input, seperator));
        };

        Ok(Point {
            x: x.parse::<isize>().map_err(|e| format!("{}: {}", x, e))?,
            y: y.parse::<isize>().map_err(|e| format!("{}: {}", y, e))?,
        })
    }

    /// Check if a point falls within the square between a and b
    ///
    /// The order of a and b does not matter
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::components::Point;
    /// assert!(Point::new(1,1).is_within(&Point::new(0,0), &Point::new(3,3)));
    /// assert!(Point::new(1,1).is_within(&Point::new(3,3), &Point::new(0,0)));
    /// ```
    pub fn is_within(&self, a: &Point, b: &Point) -> bool {
        (self.x >= a.x && self.y >= a.y && self.x <= b.x && self.y <= b.y)
            || (self.x >= b.x && self.y >= b.y && self.x <= a.x && self.y <= a.y)
    }

    /// Rotate around a point clockwise, useful to use for changing directions
    ///
    /// # Examples
    /// ````
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::RIGHT.rotate_right(), Point::DOWN);
    /// assert_eq!(Point::DOWN.rotate_right(), Point::LEFT);
    /// assert_eq!(Point::LEFT.rotate_right(), Point::UP);
    /// assert_eq!(Point::UP.rotate_right(), Point::RIGHT);
    /// ````
    pub fn rotate_right(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    /// Rotate around a point counter clockwise, useful to use for changing directions
    ///
    /// # Examples
    /// ````
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::RIGHT.rotate_left(), Point::UP);
    /// assert_eq!(Point::UP.rotate_left(), Point::LEFT);
    /// assert_eq!(Point::LEFT.rotate_left(), Point::DOWN);
    /// assert_eq!(Point::DOWN.rotate_left(), Point::RIGHT);
    /// ````
    pub fn rotate_left(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    /// Returns all neighbours directly horizontal and vertical of the point
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::new(1,1).neighbours(), vec![
    ///     Point::new(1,0),
    ///     Point::new(2,1),
    ///     Point::new(1,2),
    ///     Point::new(0,1)
    /// ]);
    /// ```
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            *self + Point::UP,
            *self + Point::RIGHT,
            *self + Point::DOWN,
            *self + Point::LEFT,
        ]
    }

    /// Returns all neighbours on the corners of the current point
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::new(1,1).diagonal_neighbours(), vec![
    ///     Point::new(2,0),
    ///     Point::new(0,0),
    ///     Point::new(2,2),
    ///     Point::new(0,2)
    /// ]);
    /// ```
    pub fn diagonal_neighbours(&self) -> Vec<Point> {
        vec![
            *self + Point::UP + Point::RIGHT,
            *self + Point::UP + Point::LEFT,
            *self + Point::DOWN + Point::RIGHT,
            *self + Point::DOWN + Point::LEFT,
        ]
    }

    /// Returns all neighbours, both square and diagonal of the current point
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::new(1,1).full_neighbours(), vec![
    ///     Point::new(1,0),
    ///     Point::new(2,1),
    ///     Point::new(1,2),
    ///     Point::new(0,1),
    ///     Point::new(2,0),
    ///     Point::new(0,0),
    ///     Point::new(2,2),
    ///     Point::new(0,2)
    /// ]);
    /// ```
    pub fn full_neighbours(&self) -> Vec<Point> {
        let mut v = self.neighbours();
        v.append(&mut self.diagonal_neighbours());

        v
    }

    /// Calculates the distance of a point to another point, now allowing for any diagonal moves
    ///
    /// # Examples
    /// ```
    /// use advent_of_code::components::Point;
    /// assert_eq!(Point::new(5,0).distance(&Point::new(10, 0)), 5);
    /// assert_eq!(Point::new(10,0).distance(&Point::new(5, 0)), 5);
    /// assert_eq!(Point::new(10,0).distance(&Point::new(5, 5)), 10);
    /// assert_eq!(Point::new(-5,0).distance(&Point::new(5, 0)), 10);
    /// assert_eq!(Point::new(5,0).distance(&Point::new(-5, 0)), 10);
    /// ````
    pub fn distance(&self, rhs: &Point) -> usize {
        let distance = *self - *rhs;
        (distance.x.abs() + distance.y.abs()) as usize
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("4,-5" , ",", Point::new(4,-5))]
    #[case("4 -5" , " ", Point::new(4,-5))]
    #[case("4--5" , "-", Point::new(4,-5))]
    #[case("-4,-5" , ",", Point::new(-4,-5))]
    #[case("0,0", ",", Point::new(0, 0))]
    #[case("1,1", ",", Point::new(1, 1))]
    #[case("1---1", "---", Point::new(1, 1))]
    fn it_can_parse_sepearted(#[case] input: &str, #[case] seperator: &str, #[case] result: Point) {
        assert_eq!(Point::parse_seperated(input, seperator).unwrap(), result)
    }

    #[rstest]
    #[case("4,-5", " ", "4,-5: seperator [ ] not found")]
    #[case("4,-a5", ",", "-a5: invalid digit found in string")]
    #[case("b4,-a5", ",", "b4: invalid digit found in string")]
    fn it_returns_an(#[case] input: &str, #[case] seperator: &str, #[case] result: String) {
        assert_eq!(Point::parse_seperated(input, seperator), Err(result.into()))
    }

    #[rstest]
    #[case(Point::new(1, 1), Point::new(2, 2), Point::new(3, 3))]
    #[case(Point::new(-2, 1), Point::new(2, 2), Point::new(0, 3))]
    #[case(Point::new(100, 10000), Point::new(0, 0), Point::new(100, 10000))]
    #[case(Point::new(200, 10000), Point::new(200, 200), Point::new(400, 10200))]
    fn it_can_add(#[case] mut input: Point, #[case] rhs: Point, #[case] result: Point) {
        assert_eq!(input + rhs, result);
        // Using add assign should also work
        input += rhs;
        assert_eq!(input, result);
    }

    #[rstest]
    #[case(Point::new(1, 1), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(1, 0), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(0, 1), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(3, 1), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(1, 3), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(1, 1), Point::new(3, 3), Point::new(0, 0))]
    #[case(Point::new(1, 0), Point::new(3, 3), Point::new(0, 0))]
    #[case(Point::new(0, 1), Point::new(3, 3), Point::new(0, 0))]
    #[case(Point::new(3, 1), Point::new(3, 3), Point::new(0, 0))]
    #[case(Point::new(1, 3), Point::new(3, 3), Point::new(0, 0))]
    #[case(Point::new(300, 300), Point::new(100, 100), Point::new(500, 500))]
    #[case(Point::new(-300, -300), Point::new(-100, -100), Point::new(-500, -500))]
    fn it_correctly_checks_if_within(#[case] input: Point, #[case] a: Point, #[case] b: Point) {
        assert!(input.is_within(&a, &b));
    }

    #[rstest]
    #[case(Point::new(-1, 0), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(5, 0), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(-3, -3), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(3, 4), Point::new(0, 0), Point::new(3, 3))]
    #[case(Point::new(0, 300), Point::new(100, 100), Point::new(500, 500))]
    #[case(Point::new(-501, -300), Point::new(-100, -100), Point::new(-500, -500))]
    fn it_correctly_checks_if_not_within(#[case] input: Point, #[case] a: Point, #[case] b: Point) {
        assert!(!input.is_within(&a, &b));
    }
}
