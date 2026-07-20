use std::ops::{Add, AddAssign, Index, IndexMut};

const UP: Point = Point::new(0, -1);
const DOWN: Point = Point::new(0, 1);

const LEFT_UP: Point = Point::new(-1, -1);
const LEFT: Point = Point::new(-1, 0);
const LEFT_DOWN: Point = Point::new(-1, 1);

const RIGHT_UP: Point = Point::new(1, -1);
const RIGHT: Point = Point::new(1, 0);
const RIGHT_DOWN: Point = Point::new(1, 1);

pub const ADJACENT: [Point; 8] = [
    LEFT_UP, UP, RIGHT_UP, RIGHT, RIGHT_DOWN, DOWN, LEFT_DOWN, LEFT,
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let bytes_input: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

        let width = bytes_input[0].len();
        let height = bytes_input.len();
        let data = bytes_input.concat();

        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn contains(&self, point: Point) -> bool {
        let Ok(x) = usize::try_from(point.x) else {
            return false;
        };

        let Ok(y) = usize::try_from(point.y) else {
            return false;
        };

        x < self.width && y < self.height
    }

    #[inline]
    pub fn get(&self, point: Point) -> Option<u8> {
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;

        if self.contains(point) {
            return self.data.get(y * self.width + x).copied();
        }

        None
    }
}

impl Index<Point> for Grid {
    type Output = u8;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        let x = usize::try_from(point.x).expect("negative x coordinate");
        let y = usize::try_from(point.y).expect("negative y coordinate");

        &self.data[y * self.width + x]
    }
}

impl IndexMut<Point> for Grid {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let x = usize::try_from(point.x).expect("negative x coordinate");
        let y = usize::try_from(point.y).expect("negative y coordinate");

        &mut self.data[y * self.width + x]
    }
}
