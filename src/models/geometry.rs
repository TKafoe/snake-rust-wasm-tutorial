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

    pub fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn rotate(&self, angle: f32) -> Point {
        let x = self.x as f32;
        let y = self.y as f32;
        let sin = angle.sin();
        let cos = angle.cos();
        Point {
            x: (x * cos - y * sin).round() as i32,
            y: (x * sin + y * cos).round() as i32,
        }
    }

    pub fn rotate_around(&self, angle: f32, center: Point) -> Point {
        let translated = self.sub(center);
        let rotated = translated.rotate(angle);
        rotated.add(center)
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
