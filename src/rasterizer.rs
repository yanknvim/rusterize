use crate::{
    framebuffer::FrameBuffer,
    vec::{IVec2, Vec2, Vec4},
};

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
        Self { fb }
    }

    pub fn clear(&mut self, col: Vec4) {
        let col = col.to_u32();
        self.fb.clear(col);
    }

    pub fn buf(&self) -> &Vec<u32> {
        self.fb.buf()
    }

    pub fn set_pixel(&mut self, p: Vec2, col: Vec4) {
        let p = self.fb.ndc_to_screen(p);
        let col = col.to_u32();

        self.fb.set_pixel(p, col);
    }

    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, col: Vec4) {
        let p0 = self.fb.ndc_to_screen(p0);
        let p1 = self.fb.ndc_to_screen(p1);
        let col = col.to_u32();

        self.draw_line_screen(p0, p1, col)
    }

    pub fn draw_line_screen(&mut self, mut p0: IVec2, p1: IVec2, col: u32) {
        let dx = (p1.x - p0.x).abs();
        let dy = (p1.y - p0.y).abs();

        let sx = if p0.x < p1.x { 1 } else { -1 };
        let sy = if p0.y < p1.y { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.fb.set_pixel(IVec2::new(p0.x, p0.y), col);
            if p0.x == p1.x && p0.y == p1.y {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                p0.x += sx;
            }
            if e2 < dx {
                err += dx;
                p0.y += sy;
            }
        }
    }

    pub fn draw_triangle_wire(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, col: Vec4) {
        self.draw_line(p0, p1, col);
        self.draw_line(p1, p2, col);
        self.draw_line(p0, p2, col);
    }

    pub fn draw_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex) {
        let p0 = self.fb.ndc_to_screen(v0.pos);
        let p1 = self.fb.ndc_to_screen(v1.pos);
        let p2 = self.fb.ndc_to_screen(v2.pos);

        self.draw_triangle_screen(p0, p1, p2, v0, v1, v2);
    }

    pub fn draw_triangle_screen(
        &mut self,
        p0: IVec2,
        p1: IVec2,
        p2: IVec2,
        v0: Vertex,
        v1: Vertex,
        v2: Vertex,
    ) {
        let min_x = p0.x.min(p1.x).min(p2.x);
        let max_x = p0.x.max(p1.x).max(p2.x);
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_y = p0.y.max(p1.y).max(p2.y);

        let p0f = Vec2::new(p0.x as f32, p0.y as f32);
        let p1f = Vec2::new(p1.x as f32, p1.y as f32);
        let p2f = Vec2::new(p2.x as f32, p2.y as f32);

        let area = Vec2::edge(p0f, p1f, p2f);

        if area == 0.0 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32, y as f32);

                let w0 = Vec2::edge(p1f, p2f, p) / area;
                let w1 = Vec2::edge(p2f, p0f, p) / area;
                let w2 = Vec2::edge(p0f, p1f, p) / area;

                if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                    let col = v0.col * w0 + v1.col * w1 + v2.col * w2;

                    self.fb.set_pixel(IVec2::new(x, y), col.to_u32());
                }
            }
        }
    }
}
