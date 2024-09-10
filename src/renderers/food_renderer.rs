use crate::models::food::Food;
use crate::palette::set_draw_color;
use crate::wasm4::{self, line};

pub struct FoodRenderer {
    frame: u32,
}

impl FoodRenderer {
    const ANIMATION_SPEED: u32 = 60;
    const ANIMATION_FRAMES: u32 = 2;
    
    const ANIMATION: [[u8; 8]; 4] = [
        [0b11000011, 0b10000001, 0b00100100, 0b00100100, 0b00000000, 0b00100100, 0b10011001, 0b11000011],
        [0b11000011, 0b10000001, 0b00100100, 0b00100100, 0b00000000, 0b00100100, 0b10011001, 0b11000011],
        [0b11000011, 0b10000001, 0b00100100, 0b00100100, 0b00000000, 0b00100100, 0b10011001, 0b11000011],
        [0b11000011, 0b10000001, 0b00100100, 0b00100100, 0b00000000, 0b00100100, 0b10011001, 0b11000011], 
    ];


    pub fn new() -> FoodRenderer {
        FoodRenderer { frame: 0 }
    }

    pub fn render(&mut self, food: &Food) {
        self.frame += 1;

        let animation_index = (self.frame % FoodRenderer::ANIMATION_SPEED) / (FoodRenderer::ANIMATION_SPEED / FoodRenderer::ANIMATION_FRAMES);
        

        let rect_size: u32;
        let offset: u32; 

        if animation_index == 0 {
            set_draw_color(0, 2);
            set_draw_color(1, 2);
            rect_size = 2; 
        } else {
            set_draw_color(0, 3);
            set_draw_color(1, 3);
            rect_size = 3; 
        }
        offset = (8 - rect_size) / 2;

        wasm4::rect(
            food.loc.y * 8 + offset as i32,
            food.loc.x * 8 + offset as i32, rect_size, rect_size);
    }

    fn draw_frame(&self, frame: u32) {
        let animation_index = (frame % FoodRenderer::ANIMATION_SPEED) / (FoodRenderer::ANIMATION_SPEED / FoodRenderer::ANIMATION_FRAMES);
        let rect_size: u32;
        let offset: u32; 
        
        
        set_draw_color(0, 2);
        set_draw_color(1, 2);

        let animation_coordinates = FoodRenderer::ANIMATION[0][animation_index as usize];

    }
}
