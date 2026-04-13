use crate::{framebuffer::FrameBuffer, vec::{Vec2, Vec4}};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec2,
    pub col: Vec4,
}

pub struct Rasterizer<'a> {
    fb: &'a mut FrameBuffer,
}

impl<'a> Rasterizer<'a> {
    pub fn new(fb: &'a mut FrameBuffer) -> Self {
        Self {
            fb
        }
    }

    pub fn clear(&mut self, col: Vec4) {
        let col = col.to_u32();
        self.fb.clear(col);
    }

    pub fn buf(&self) -> &Vec<u32> {
        self.fb.buf()
    }

    pub fn set_pixel(&mut self, p: Vec2, col: Vec4) {
        let (x, y) = self.fb.ndc_to_screen(p);
        let col = col.to_u32();

        self.fb.set_pixel(x, y, col);
    }

    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, col: Vec4) {
        let (mut x0, mut y0) = self.fb.ndc_to_screen(p0);
        let (x1, y1) = self.fb.ndc_to_screen(p1);
        let col = col.to_u32();

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.fb.set_pixel(x0, y0, col);
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

    pub fn draw_triangle_wire(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, col: Vec4) {
        self.draw_line(p0, p1, col);
        self.draw_line(p1, p2, col);
        self.draw_line(p0, p2, col);
    }

    pub fn draw_triangle(
        &mut self,
        v0: Vertex,
        v1: Vertex,
        v2: Vertex,
    ) {
        let (x0, y0) = self.fb.ndc_to_screen(v0.pos);
        let (x1, y1) = self.fb.ndc_to_screen(v1.pos);
        let (x2, y2) = self.fb.ndc_to_screen(v2.pos);

        let p0 = Vec2::new(x0 as f32, y0 as f32);
        let p1 = Vec2::new(x1 as f32, y1 as f32);
        let p2 = Vec2::new(x2 as f32, y2 as f32);

        let min_x = x0.min(x1).min(x2);
        let max_x = x0.max(x1).max(x2);
        let min_y = y0.min(y1).min(y2);
        let max_y = y0.max(y1).max(y2);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32, y as f32);

                let area = Vec2::edge(p0, p1, p2);

                let w0 = Vec2::edge(p1, p2, p) / area;
                let w1 = Vec2::edge(p2, p0, p) / area;
                let w2 = Vec2::edge(p0, p1, p) / area;

                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) 
                || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0){
                    let col = v0.col * w0 + v1.col * w1 + v2.col * w2;
                    self.fb.set_pixel(x, y, col.to_u32());
                }
            }
        }
    }
}
