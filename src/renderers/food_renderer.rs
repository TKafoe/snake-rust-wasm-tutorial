use crate::models::food::Food;
use crate::palette::set_draw_color;
use crate::wasm4;

pub struct FoodRenderer {}

impl FoodRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(food: &Food) { 
        set_draw_color(0, 3);
    
        wasm4::rect(food.loc.x * 4 + 2, food.loc.y * 4 + 2, 4, 4);
    }
}
