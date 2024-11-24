use crate::wasm4;
use crate::models::geometry::{Direction, Point};
use crate::

struct SnakeController {
    input_controller: InputController,
}


impl SnakeController {
    pub fn new() -> Self {
        Self {
            input_controller: InputController::new(),
        }
    }

    pub fn process_input(&mut self, snake: &Snake) {
         if self.input_controller.is_pressed(wasm4::BUTTON_UP) &&
             snake.head().sub(snake.body[1]).y != 1 {
                 snake.set_direction(Point { x: 0, y: -1 });
         } else if self.input_controller.is_pressed(wasm4::BUTTON_DOWN) && 
             self.head().sub(self.body[1]).y != -1 {
                 snake.set_direction(Point { x: 0, y: 1 });
         } else if self.input_controller.is_pressed(wasm4::BUTTON_LEFT) &&
            self.snake.set_direction(Direction::Left);
        } else if self.input_controller.is_pressed(wasm4::BUTTON_RIGHT) {
            self.snake.set_direction(Direction::Right);
        }
    


        match direction {
            Direction::Up => {
                if self.head().sub(self.body[1]).y != 1 {
                    self.direction = Point { x: 0, y: -1 };
                }
            }
            Direction::Down => {
                if             }
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


    pub fn update(&mut self, snake: &Snake, frame_count: u32) {
        // Process input
        self.process_input(snake);        

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
