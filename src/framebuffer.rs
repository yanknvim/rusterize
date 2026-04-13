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

    pub fn buf(&self) -> &Vec<u32> {
        &self.buf
    }

    pub fn clear(&mut self, col: u32) {
        self.buf.fill(col)
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, col: u32) {
        if x < 0 || y < 0 {
            return;
        }

        if x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        self.buf[(y * self.width as i32 + x) as usize] = col;
    }

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, col: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set_pixel(x0, y0, col);
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
