use crate::wasm4;
use crate::models::snake::Snake;
use crate::models::geometry::Point;
use crate::palette::set_draw_color;

pub struct SnakeRenderer {}

impl SnakeRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, snake: &Snake) {
        set_draw_color(0x43);
        for &Point { x, y } in snake.body.iter() {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }
        
        set_draw_color(0x4);
        wasm4::rect(snake.body[0].x * 8, snake.body[0].y * 8, 8, 8);
    }
}
