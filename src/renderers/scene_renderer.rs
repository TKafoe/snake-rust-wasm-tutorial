use crate::palette::set_draw_color;
use crate::{models::geometry::WALL_SIZE, wasm4};

pub struct SceneRenderer {}

impl SceneRenderer {
    fn render_wall() {
        set_draw_color(0, 3);
        set_draw_color(1, 3);

        let wall_size: i32 = WALL_SIZE * 8;
        let top_right_x: i32 = 160 - wall_size;
        let wall_size_u: u32 = wall_size as u32;

        wasm4::rect(0, 0, wall_size_u, 160);
        wasm4::rect(0, 0, 160, wall_size_u);
        wasm4::rect(top_right_x, 0, wall_size_u, 160);
        wasm4::rect(0, top_right_x, 160, wall_size_u);
    }

    pub fn render() {
        SceneRenderer::render_wall();
    }
}
