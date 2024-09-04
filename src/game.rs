use crate::models::snake::Snake;
use crate::renderers::snake_renderer::SnakeRenderer;

pub struct Game {
    snake: Snake,
    snake_renderer: SnakeRenderer,
    frame_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            snake_renderer: SnakeRenderer::new(),
            frame_count: 0, 
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count % 15 == 0 {
            self.snake.update();
        }
        self.snake_renderer.render(&self.snake);
    }
}
