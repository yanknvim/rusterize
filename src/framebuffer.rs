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

    pub fn set_pixel(&mut self, x: usize, y: usize, col: u32) {
        if x >= self.width || y >= self.height { return; }

        self.buf[x * self.width + y] = col;
    }
}
