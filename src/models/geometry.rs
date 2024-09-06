#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
