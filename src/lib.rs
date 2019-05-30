use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    buf: Vec<u8>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn buf(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = (row * self.width + col) as usize;

                self.buf[4 * i] = 2 * (i % 111) as u8; // r
                self.buf[4 * i + 1] = 2 * (row % 100) as u8; // g
                self.buf[4 * i + 2] = 2 * (col % 100) as u8; // b
                self.buf[4 * i + 3] = 255; // alpha
            }
        }
    }

    pub fn new() -> Canvas {
        let width = 400u32;
        let height = 300u32;
        let buf = vec![0; (width * height) as usize * 4];
        Canvas { width, height, buf }
    }
}
