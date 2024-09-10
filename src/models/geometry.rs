#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub static PLAYING_FIELD_SIZE: i32 = 19;
pub static WALL_SIZE: i32 = 1;
