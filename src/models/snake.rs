use crate::models::geometry::{Point, Direction};

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 8, y: 6},
                Point { x: 8, y: 5},
                Point { x: 8, y: 4},
            ],
            direction: Point { x: 0, y: 1},
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.direction.y != 1 {
                    self.direction = Point { x: 0, y: -1 };
                }
            }
            Direction::Down => {
                if self.direction.y != -1 {
                    self.direction = Point { x: 0, y: 1 };
                }
            }
            Direction::Left => {
                if self.direction.x != 1 {
                    self.direction = Point { x: -1, y: 0 };
                }
            }
            Direction::Right => {
                if self.direction.x != -1 {
                    self.direction = Point { x: 1, y: 0 };
                }
            }
        }
    }

    pub fn update(&mut self) -> Option<Point> {
        // Move the snake
        self.body.insert(
            0,
            Point {
                x: (self.body[0].x + self.direction.x),
                y: (self.body[0].y + self.direction.y),
            },
        );
               
        self.body.pop()
    }
}
