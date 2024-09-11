use crate::models::geometry::Point;
use crate::models::snake::Snake;
use crate::palette::set_draw_color;
use crate::wasm4;

pub struct SnakeRenderer {}

impl SnakeRenderer {
    pub fn render(snake: &Snake) {
        set_draw_color(0, 3);
        set_draw_color(1, 1);
        for ( i, &Point { x, y }) in snake.body.iter().enumerate() {
            if snake.highlight_food >= 0 && i as i32 == snake.highlight_food as i32 {
                set_draw_color(0, 2);
            } else {
                set_draw_color(0, 3);
            }

            wasm4::rect(x * 8, y * 8, 8, 8);
        }
    }
}
