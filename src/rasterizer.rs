use crate::{
    framebuffer::FrameBuffer,
    vec::{IVec2, Vec2, Vec4},
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec2,
    pub z: f32,
    pub col: Vec4,
}

pub struct Rasterizer {}

impl Rasterizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clear(&mut self, fb: &mut FrameBuffer, col: Vec4) {
        let col = col.to_u32();
        fb.clear(col);
    }

    pub fn set_pixel(&mut self, fb: &mut FrameBuffer, p: Vec2, col: Vec4) {
        let p = fb.ndc_to_screen(p);
        let col = col.to_u32();

        fb.set_pixel(p, 0.0, col);
    }

    pub fn draw_line(&mut self, fb: &mut FrameBuffer, p0: Vec2, p1: Vec2, col: Vec4) {
        let p0 = fb.ndc_to_screen(p0);
        let p1 = fb.ndc_to_screen(p1);
        let col = col.to_u32();

        self.draw_line_screen(fb, p0, p1, col)
    }

    pub fn draw_line_screen(&mut self, fb: &mut FrameBuffer, mut p0: IVec2, p1: IVec2, col: u32) {
        let dx = (p1.x - p0.x).abs();
        let dy = (p1.y - p0.y).abs();

        let sx = if p0.x < p1.x { 1 } else { -1 };
        let sy = if p0.y < p1.y { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            fb.set_pixel(IVec2::new(p0.x, p0.y), 0.0, col);
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

    pub fn draw_triangle_wire(
        &mut self,
        fb: &mut FrameBuffer,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        col: Vec4,
    ) {
        self.draw_line(fb, p0, p1, col);
        self.draw_line(fb, p1, p2, col);
        self.draw_line(fb, p0, p2, col);
    }

    pub fn draw_triangle(&mut self, fb: &mut FrameBuffer, v0: Vertex, v1: Vertex, v2: Vertex) {
        let p0 = fb.ndc_to_screen(v0.pos);
        let p1 = fb.ndc_to_screen(v1.pos);
        let p2 = fb.ndc_to_screen(v2.pos);

        self.draw_triangle_screen(fb, p0, p1, p2, v0, v1, v2);
    }

    pub fn draw_triangle_screen(
        &mut self,
        fb: &mut FrameBuffer,
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

        let area = IVec2::edge(p0, p1, p2.to_vec2());

        if area == 0.0 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32, y as f32);

                let w0 = IVec2::edge(p1, p2, p) / area;
                let w1 = IVec2::edge(p2, p0, p) / area;
                let w2 = IVec2::edge(p0, p1, p) / area;

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let col = v0.col * w0 + v1.col * w1 + v2.col * w2;
                    let z = v0.z * w0 + v1.z * w1 + v2.z * w2;

                    fb.set_pixel(IVec2::new(x, y), z, col.to_u32());
                }
            }
        }
    }
}
