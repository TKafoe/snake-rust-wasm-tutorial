use crate::models::geometry::Point;
use crate::models::snake::Snake;
use fastrand::Rng;

pub struct Food {
    pub loc: Point,
    rng: Rng,
}

impl Food {
    pub fn new() -> Food {
        let rng = Rng::with_seed(255);

        let mut start_loc = Point {
            x: rng.i32(1..19),
            y: rng.i32(1..19),
        };

        // Check if food started in the snake. If so, re-roll the location.
        while start_loc.x == Snake::STARTING_POINT.x && start_loc.y == Snake::STARTING_POINT.y
            || start_loc.x == Snake::STARTING_POINT.x && start_loc.y == Snake::STARTING_POINT.y - 1
            || start_loc.x == Snake::STARTING_POINT.x && start_loc.y == Snake::STARTING_POINT.y - 2
        {
            start_loc = Point {
                x: rng.i32(1..19),
                y: rng.i32(1..19),
            };
        }

        Food {
            loc: start_loc,
            rng,
        }
    }

    pub fn move_loc(&mut self, snake: &Snake) {
        let mut new_loc = Point {
            x: self.rng.i32(1..19),
            y: self.rng.i32(1..19),
        };

        // Check if food is in the snake. If so, re-roll the location.
        while snake.body.contains(&new_loc) {
            new_loc = Point {
                x: self.rng.i32(1..19),
                y: self.rng.i32(1..19),
            };
        }

        self.loc = new_loc;
    }
}
