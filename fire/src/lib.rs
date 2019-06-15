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
    intbuf: Vec<u8>,
    t: u32,
    oldt: u32,
}

const COLORS: [[u8; 3]; 37] = [[  7,   7,   7],
                               [ 31,   7,   7],
                               [ 47,  15,   7],
                               [ 71,  15,   7],
                               [ 87,  23,   7],
                               [103,  31,   7],
                               [119,  31,   7],
                               [143,  39,   7],
                               [159,  47,   7],
                               [175,  63,   7],
                               [191,  71,   7],
                               [199,  71,   7],
                               [223,  79,   7],
                               [223,  87,   7],
                               [223,  87,   7],
                               [215,  95,   7],
                               [215,  95,   7],
                               [215, 103,  15],
                               [207, 111,  15],
                               [207, 119,  15],
                               [207, 127,  15],
                               [207, 135,  23],
                               [199, 135,  23],
                               [199, 143,  23],
                               [199, 151,  31],
                               [191, 159,  31],
                               [191, 159,  31],
                               [191, 167,  39],
                               [191, 167,  39],
                               [191, 175,  47],
                               [183, 175,  47],
                               [183, 183,  47],
                               [183, 183,  55],
                               [207, 207, 111],
                               [223, 223, 159],
                               [239, 239, 199],
                               [255, 255, 255]];

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
        if self.t < self.oldt + 50 {
            return;
        }
        self.oldt = self.t;
        for col in 0..self.width {
            for row in 1..self.height {
                let i = (row * self.width + col) as usize;
                let pixel = self.intbuf[i];
                if pixel == 0 {
                    self.intbuf[((row-1) * self.width + col) as usize] = 0;
                } else {
                    let r: u32 = js_sys::Math::round(js_sys::Math::random()*4.0) as u32 - 2;

                    let dstcol = (col + self.width - r) % self.width;
                    let decrease = if js_sys::Math::random() > 0.5 { 1 } else { 0 };
                    self.intbuf[((row-1) * self.width + dstcol) as usize] =
                        if self.intbuf[i] > decrease { self.intbuf[i] - decrease } else { 0 };
                }
            }
        }

        for col in 0..self.width {
            for row in 0..self.height {
                let i = (row * self.width + col) as usize;
                self.buf[i * 4 + 0] = COLORS[self.intbuf[i] as usize][0];
                self.buf[i * 4 + 1] = COLORS[self.intbuf[i] as usize][1];
                self.buf[i * 4 + 2] = COLORS[self.intbuf[i] as usize][2];
                self.buf[i * 4 + 3] = 255;
            }
        }
    }

    pub fn new() -> Canvas {
        console_error_panic_hook::set_once();
        let width = 128u32;
        let height = 128u32;
        let mut buf = vec![0; (width * height) as usize * 4];

        let mut intbuf = vec![0; (width * height) as usize];
        for col in 30..98 {
            intbuf[((height-1)*width + col) as usize] = 36;
        }
        let t = 0;
        let oldt = t;
        Canvas { width, height, buf, intbuf, t, oldt}
    }
}
