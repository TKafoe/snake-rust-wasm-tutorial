use crate::models::food::Food;
use crate::models::geometry::Direction;
use crate::models::snake::Snake;
use crate::renderers::food_renderer::FoodRenderer;
use crate::renderers::scene_renderer::SceneRenderer;
use crate::renderers::snake_renderer::SnakeRenderer;
use crate::utils::input_controller::InputController;
use crate::wasm4;

pub struct Game {
    input_controller: InputController,
    scene_renderer: SceneRenderer,
    food_renderer: FoodRenderer,
    snake: Snake,
    food: Food,
    frame_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            input_controller: InputController::new(),
            scene_renderer: SceneRenderer::new(),
            food_renderer: FoodRenderer::new(),
            snake: Snake::new(),
            frame_count: 0,
            food: Food::new(),
        }
    }

    pub fn process_input(&mut self) {
   }

    pub fn update(&mut self) {
        // Increase the frame count
        self.frame_count += 1;

        // Process input
        self.process_input();

        // Update the snake every 15 frames
        if self.frame_count % 10 == 0 {
            self.snake.update();
            self.food.update(&self.snake);

            // Check collision snake and food
            if self.snake.head() == self.food.loc {
                self.snake.grow();
                self.food.move_loc(&self.snake);
            }
        }

        if !self.snake.is_dead() {
            // Render the snake
            SnakeRenderer::render(&self.snake);
        }

        // Render the scene
        self.scene_renderer.render();

        // Render the food
        self.food_renderer.render(&self.food);

        // Save the current state of the gamepad for the next frame
        self.input_controller.save_state();

        // Check if the snake is dead
        if self.snake.is_dead() {
            self.snake = Snake::new();
            self.food = Food::new();
        }
    }
}
