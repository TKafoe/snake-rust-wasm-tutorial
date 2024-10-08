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

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.head().sub(self.body[1]).y != 1 {
                    self.direction = Point { x: 0, y: -1 };
                }
            }
            Direction::Down => {
                if self.head().sub(self.body[1]).y != -1 {
                    self.direction = Point { x: 0, y: 1 };
                }
            }
            Direction::Left => {
                if self.head().sub(self.body[1]).x != 1 {
                    self.direction = Point { x: -1, y: 0 };
                }
            }
            Direction::Right => {
                if self.head().sub(self.body[1]).x != -1 {
                    self.direction = Point { x: 1, y: 0 };
                }
            }
        }
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

    pub fn update(&mut self) {
        // Move the snake
        self.body.insert(
            0,
            Point {
                x: (self.body[0].x + self.direction.x),
                y: (self.body[0].y + self.direction.y),
            },
        );

        self.body.pop();

        if self.check_collision_with_self() || self.check_collision_with_wall() {
            self.is_dead = true;
        }
        
        if self.highlight_food >= 0 {
            self.highlight_food += 1;
            if self.highlight_food >= self.body.len() as i32 {
                self.highlight_food = -1;
            }
        }
    }

    pub fn head(&self) -> Point {
        self.body[0]
    }

    pub fn grow(&mut self) {
        if self.body.len() == 0 {
            return;
        }

        self.highlight_food = 0;

        let tail = self.body.last().unwrap();
        self.body.push(Point {
            x: tail.x,
            y: tail.y,
        });
    }
}
