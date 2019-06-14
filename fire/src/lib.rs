use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    buf: Vec<u8>,
    t: u32,
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

    pub fn update(&mut self, t: u32) {
        self.t = t;
    }

    pub fn draw(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = (row * self.width + col) as usize;

                self.buf[4 * i]     = (row ^ col) as u8; // r
                self.buf[4 * i + 1] = ((self.t/10+row) ^ ((self.t / 100)+col)) as u8; // g
                self.buf[4 * i + 2] = (self.t / 10000) as u8; // b
                self.buf[4 * i + 3] = 255 as u8; // alpha
            }
        }
    }

    pub fn new() -> Canvas {
        let width = 400u32;
        let height = 300u32;
        let buf = vec![0; (width * height) as usize * 4];
        let t = 0;
        Canvas { width, height, buf, t }
    }
}
