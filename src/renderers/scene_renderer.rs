use crate::models::geometry::Point;
use crate::palette::set_draw_color;
use crate::wasm4::{blit, line, BLIT_1BPP};
use crate::{models::geometry::WALL_SIZE, wasm4};

pub struct SceneRenderer {
    frame: u32,
}

impl SceneRenderer {
    pub fn new() -> SceneRenderer {
        SceneRenderer { frame: 0 }
    }

    fn render_wall() {
        set_draw_color(0, 4);
        set_draw_color(1, 4);

        let wall_size: i32 = WALL_SIZE * 8;
        let top_right_x: i32 = 160 - wall_size;
        let wall_size_u: u32 = wall_size as u32;

        wasm4::rect(0, 0, wall_size_u, 160);
        wasm4::rect(0, 0, 160, wall_size_u);
        wasm4::rect(top_right_x, 0, wall_size_u, 160);
        wasm4::rect(0, top_right_x, 160, wall_size_u);
    }

    pub fn render(&mut self) {
        self.frame += 1;

        SceneRenderer::render_wall();
        
        set_draw_color(0, 1);
        set_draw_color(1, 2);
        const smiley: [u8; 8] = [
               0b11000011,
                   0b10000001,
                       0b00100100,
                           0b00100100,
                               0b00000000,
                                   0b00100100,
                                       0b10011001,
                                           0b11000011,
        ];

        blit(&smiley, 8, 8, 8, 8, BLIT_1BPP);
        
        
        // Rectangle points
        let rectangle_size = 2;
        let center_point = Point { x: 30, y: 30 };
        let mut p1 = Point { x: center_point.x - rectangle_size, y: center_point.y - rectangle_size }; 
        let mut p2 = Point { x: center_point.x + rectangle_size, y: center_point.y - rectangle_size };
        let mut p3 = Point { x: center_point.x + rectangle_size, y: center_point.y + rectangle_size };
        let mut p4 = Point { x: center_point.x - rectangle_size, y: center_point.y + rectangle_size };
            
        // Rotate the rectangle
        p1 = p1.rotate_around((self.frame % 240) as f32 / 240.0 * 6.28, center_point);
        p2 = p2.rotate_around((self.frame % 240) as f32 / 240.0 * 6.28, center_point); 
        p3 = p3.rotate_around((self.frame % 240) as f32 / 240.0 * 6.28, center_point);
        p4 = p4.rotate_around((self.frame % 240) as f32 / 240.0 * 6.28, center_point);


        set_draw_color(0, 3);
        line(p1.x, p1.y, p2.x, p2.y);
        line(p2.x, p2.y, p3.x, p3.y);
        line(p3.x, p3.y, p4.x, p4.y);
        line(p4.x, p4.y, p1.x, p1.y); 
    }
}
