use crate::vec::{IVec2, Vec2};

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    buf: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buf: vec![0; width * height],
        }
    }

    pub fn ndc_to_screen(&self, v: Vec2) -> IVec2 {
        let scale = self.width.max(self.height) as f32;

        let offset_x = (self.width as f32 - scale) * 0.5;
        let offset_y = (self.height as f32 - scale) * 0.5;

        let x = (v.x + 1.0) * 0.5 * scale + offset_x;
        let y = (1.0 - (v.y + 1.0) * 0.5) * scale + offset_y;

        IVec2::new(x as i32, y as i32)
    }

    pub fn buf(&self) -> &Vec<u32> {
        &self.buf
    }

    pub fn clear(&mut self, col: u32) {
        self.buf.fill(col)
    }

    pub fn set_pixel(&mut self, p: IVec2, col: u32) {
        if p.x < 0 || p.y < 0 {
            return;
        }

        if p.x >= self.width as i32 || p.y >= self.height as i32 {
            return;
        }

        self.buf[(p.y * self.width as i32 + p.x) as usize] = col;
    }
}
