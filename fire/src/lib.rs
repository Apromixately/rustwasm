use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[wasm_bindgen]
extern {
#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    buf: Vec<u8>,
    t: u32,
    oldt: u32,
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
        //if self.t < self.oldt + 1000 {
        //    return;
        //}
        self.oldt = self.t;
        for row in 1..self.height {
            for col in 5..self.width-5 {
                let offset: u32 = if js_sys::Math::random() < 0.5 { 1 } else { self.width-1 };
                let newcol = (col + offset) % self.width;

                let i = (row * self.width + col) as usize;
                let above = ((row-1) * self.width + newcol) as usize;

                let rand = js_sys::Math::random();
                let step = if rand < 0.03 {
                    18
                } else if rand < 0.5 {
                    3
                } else {
                    0
                };

                let oldval = self.buf[4*i];
                let newval = if oldval > step { oldval - step } else { 0 };
                self.buf[4 * above + 0] = newval;
                self.buf[4 * above + 1] = newval;
                self.buf[4 * above + 2] = newval;
            }
        }
    }

    pub fn new() -> Canvas {
        console_error_panic_hook::set_once();
        let width = 400u32;
        let height = 300u32;
        let mut buf = vec![0; (width * height) as usize * 4];
        for row in 0..height {
            for col in 0..width {
                let i = (row*width + col) as usize;
                buf[4 * i + 0] = if row == height-1 && col > 125 && col < 275 { 255 } else { 0 };
                buf[4 * i + 1] = if row == height-1 && col > 125 && col < 275 { 255 } else { 0 };
                buf[4 * i + 2] = if row == height-1 && col > 125 && col < 275 { 255 } else { 0 };

                buf[4 * i + 3] = 255;
            }
        }
        let t = 0;
        let oldt = t;
        Canvas { width, height, buf, t, oldt}
    }
}
