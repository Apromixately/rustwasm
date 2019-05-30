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

    pub fn draw(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.buf[4*i] = (i % 255) as u8; // r
            self.buf[4*i+1] = 0; // g
            self.buf[4*i+2] = 0; // b
            self.buf[4*i+3] = 255; // alpha
        }
    }

    pub fn new() -> Canvas {
        let width = 400u32;
        let height = 300u32;
        let buf = vec![0; (width*height) as usize * 4];
        Canvas {
            width,
            height,
            buf,
        }
    }
}
