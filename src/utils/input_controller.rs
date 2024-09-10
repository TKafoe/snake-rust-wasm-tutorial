use crate::wasm4;

pub struct InputController {
    prev_state: u8,
}

impl InputController {
    pub fn new() -> InputController {
        InputController { prev_state: 0 }
    }

    pub fn save_state(&mut self) {
        self.prev_state = unsafe { *wasm4::GAMEPAD1 };
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.prev_state & key != 0
    }
}
