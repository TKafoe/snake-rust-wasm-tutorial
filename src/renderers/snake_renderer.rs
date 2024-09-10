use crate::models::geometry::Point;
use crate::models::snake::Snake;
use crate::palette::set_draw_color;
use crate::wasm4;

pub struct SnakeRenderer {}

impl SnakeRenderer {
    pub fn render(snake: &Snake) {
        set_draw_color(0, 3);
        set_draw_color(1, 1);
        for &Point { x, y } in snake.body.iter() {
            wasm4::rect(x * 8, y * 8, 8, 8);
        }

        wasm4::rect(snake.body[0].x * 8, snake.body[0].y * 8, 8, 8);
    }
}
