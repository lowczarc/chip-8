use minifb::{Window, WindowOptions};

pub struct Display {
    window: Window,
    framebuffer: [u32; 64 * 32],
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
        self.window
            .update_with_buffer(&self.framebuffer, 64, 32)
            .unwrap();
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

        self.window
            .update_with_buffer(&self.framebuffer, 64, 32)
            .unwrap();

        return col;
    }
}
