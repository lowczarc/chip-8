use minifb::{Window, WindowOptions};

pub struct Display {
    window: Window,
    framebuffer: [u32; 64 * 32],
}

fn key_to_minifb_key(key: u8) -> Option<minifb::Key> {
    match key {
        0x0 => Some(minifb::Key::Key0),
        0x1 => Some(minifb::Key::Key1),
        0x2 => Some(minifb::Key::Key2),
        0x3 => Some(minifb::Key::Key3),
        0x4 => Some(minifb::Key::Key4),
        0x5 => Some(minifb::Key::Key5),
        0x6 => Some(minifb::Key::Key6),
        0x7 => Some(minifb::Key::Key7),
        0x8 => Some(minifb::Key::Key8),
        0x9 => Some(minifb::Key::Key9),
        0xA => Some(minifb::Key::A),
        0xB => Some(minifb::Key::B),
        0xC => Some(minifb::Key::C),
        0xD => Some(minifb::Key::D),
        0xE => Some(minifb::Key::E),
        0xF => Some(minifb::Key::F),
        _ => None,
    }
}

impl Display {
    pub fn new() -> Self {
        Self {
            window: Window::new("Chip8 Emulator", 512, 256, WindowOptions::default()).unwrap(),
            framebuffer: [0; 64 * 32],
        }
    }

    pub fn cls(&mut self) {
        self.framebuffer = [0; 64 * 32];
    }

    pub fn draw(&mut self, x: u8, y: u8, pxs: u8) -> bool {
        let mut col = false;

        for i in 0..8 {
            if ((pxs << i) & 0x80) != 0 {
                self.framebuffer[64 * (y as usize % 32) + (x as usize + i) % 64] ^= 0x00ffffff;

                if self.framebuffer[64 * (y as usize % 32) + (x as usize + i) % 64] == 0 {
                    col = true;
                }
            }
        }

        return col;
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.framebuffer, 64, 32)
            .unwrap();
    }

    pub fn check_key_press(&self, key: u8) -> bool {
        if let Some(minifb_key) = key_to_minifb_key(key) {
            self.window.is_key_down(minifb_key)
        } else {
            false
        }
    }
}
