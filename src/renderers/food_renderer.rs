use crate::models::food::Food;
use crate::palette::set_draw_color;
use crate::wasm4;

pub struct FoodRenderer {}

impl FoodRenderer {
    pub fn render(food: &Food) {
        set_draw_color(0, 4);
        set_draw_color(1, 4);
        wasm4::rect(food.loc.x * 8 + 2, food.loc.y * 8 + 2, 4, 4);
    }
}
