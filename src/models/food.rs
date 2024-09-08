use crate::models::geometry::{Point, get_random_grid};

pub struct Food {
    pub loc: Point,
}

impl Food {
    pub fn new() -> Food {
        Food {
            loc: get_random_grid()
        }
    }

    pub fn change_to_random_loc(&mut self) {
       self.loc = get_random_grid(); 
    }
}
