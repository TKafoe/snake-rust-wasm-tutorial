use crate::models::geometry::Point;
use fastrand::Rng;

pub struct Food {
    pub loc: Point,
    rng: Rng,
}

impl Food {
    pub fn new() -> Food {
        let rng = Rng::with_seed(255);
        Food {
            loc: Point {
                x: rng.i32(0..20),
                y: rng.i32(0..20),
            },
            rng, 
        }
    }

    pub fn move_loc(&mut self) {
       self.loc = Point {
           x: self.rng.i32(0..20),
           y: self.rng.i32(0..20),
       }; 
    }
}
