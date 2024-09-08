use std::time::{Instant, Duration};

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

pub struct Random {
    start: Instant,
}

impl Random {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn get_random_grid(&self) -> Point {
        Point { x: self.start.elapsed().subsec_nanos() as u32 % 20, y: self.start.elapsed() as u32 % 20}
    }
}
