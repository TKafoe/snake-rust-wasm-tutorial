use crate::models::food::Food;
use crate::palette::set_draw_color;
use crate::wasm4::{blit, BLIT_1BPP};

pub struct FoodRenderer {
    frame: u32,
}

impl FoodRenderer {
    const ANIMATION_SPEED: u32 = 60;
    const ANIMATION_FRAMES: u32 = 4;

    const ANIMATION: [[u8; 8]; 4] = [
        [
            0b00000000, 0b00000000, 0b00011000, 0b00110100, 0b00101100, 0b00011000, 0b00000000,
            0b00000000,
        ],
        [
            0b00000000, 0b00000000, 0b00011000, 0b00101100, 0b00110100, 0b00011000, 0b00000000,
            0b00000000,
        ],
        [
            0b00000000, 0b00000000, 0b00011000, 0b00110100, 0b00101100, 0b00011000, 0b00000000,
            0b00000000,
        ],
        [
            0b00000000, 0b00000000, 0b00011000, 0b00101100, 0b00110100, 0b00011000, 0b00000000,
            0b00000000,
        ],
    ];

    pub fn new() -> FoodRenderer {
        FoodRenderer { frame: 0 }
    }

    pub fn render(&mut self, food: &Food) {
        self.frame += 1;

        let animation_index = (self.frame % FoodRenderer::ANIMATION_SPEED)
            / (FoodRenderer::ANIMATION_SPEED / FoodRenderer::ANIMATION_FRAMES);
        self.draw_frame(animation_index, food);
    }

    fn draw_frame(&self, frame: u32, food: &Food) {
        set_draw_color(0, 1);
        set_draw_color(1, 2);
        let frame_data = FoodRenderer::ANIMATION[frame as usize];
        blit(&frame_data, food.loc.x * 8, food.loc.y * 8, 8, 8, BLIT_1BPP);
    }
}
