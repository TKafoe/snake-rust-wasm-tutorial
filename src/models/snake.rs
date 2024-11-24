use crate::models::geometry::{Direction, Point};

use super::geometry::{PLAYING_FIELD_SIZE, WALL_SIZE};

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
    pub is_dead: bool,
    pub highlight_food: i32,
}

impl Snake {
    pub const STARTING_POINT: Point = Point { x: 8, y: 6 };

    pub fn new() -> Self {
        Self {
            body: vec![
                Self::STARTING_POINT,
                Self::STARTING_POINT.sub(Point { x: 0, y: 1 }),
                Self::STARTING_POINT.sub(Point { x: 0, y: 2 }),
            ],
            direction: Point { x: 0, y: 1 },
            is_dead: false,
            highlight_food: -1,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }
    
    pub fn set_direction(&mut self, direction: Point) {
        self.direction = direction;
    }

    fn check_collision_with_self(&self) -> bool {
       self.check_collision_with_object(self.head()) 
    }

    fn check_collision_with_wall(&self) -> bool {
        let head = self.head();
        head.x < WALL_SIZE
            || head.x >= PLAYING_FIELD_SIZE
            || head.y < WALL_SIZE
            || head.y >= PLAYING_FIELD_SIZE
    }

    pub fn check_collision_with_object(&self, point: Point) -> bool {
        for i in 1..self.body.len() {
            if point == self.body[i] {
                return true;
            }
        }

        false
    }
    pub fn head(&self) -> Point {
        self.body[0]
    }

}
