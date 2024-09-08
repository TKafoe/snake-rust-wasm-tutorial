use crate::wasm4;

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *wasm4::PALETTE = palette;
    }
}

fn set_draw_color_registry(idx: u16) {
    unsafe { *wasm4::DRAW_COLORS = idx.into() }
}


pub fn set_draw_color(color_id: u16, palette_color_id: u16) {
    if palette_color_id > 4 {
        return;
    }

    if color_id == 0 {
        unsafe { 
            *wasm4::DRAW_COLORS &= 0b1111_1111_1111_0000;
            *wasm4::DRAW_COLORS |= palette_color_id;
        }
    } else if color_id == 1 {
        unsafe {
            *wasm4::DRAW_COLORS &= 0b1111_1111_0000_1111;
            *wasm4::DRAW_COLORS |= palette_color_id << 4;
        }
    } else if color_id == 2 {
        unsafe {
            *wasm4::DRAW_COLORS &= 0b1111_1111_0000_1111;
            *wasm4::DRAW_COLORS |= palette_color_id << 8;
        }
    } else if color_id == 3 {
        unsafe {
            *wasm4::DRAW_COLORS &= 0b1111_1111_1111_0000;
            *wasm4::DRAW_COLORS |= palette_color_id << 12;
        }
    }
}
