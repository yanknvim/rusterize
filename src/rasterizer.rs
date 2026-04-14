use crate::{
    framebuffer::FrameBuffer,
    vec::{IVec2, Vec2, Vec3, Vec4},
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec2,
    pub z: f32,
    pub world_pos: Vec3,
    pub col: Vec4,
    pub normal: Vec3,
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
        let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
        let pos = Vec3::splat(0.0);

        let min_x = p0.x.min(p1.x).min(p2.x);
        let max_x = p0.x.max(p1.x).max(p2.x);
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_y = p0.y.max(p1.y).max(p2.y);

        let inv_z0 = 1.0 / v0.z;
        let inv_z1 = 1.0 / v1.z;
        let inv_z2 = 1.0 / v2.z;

        let n0_z = v0.normal * inv_z0;
        let n1_z = v1.normal * inv_z1;
        let n2_z = v2.normal * inv_z2;

        let col0_z = v0.col * inv_z0;
        let col1_z = v1.col * inv_z1;
        let col2_z = v2.col * inv_z2;

        let wpos0_z = v0.world_pos * inv_z0;
        let wpos1_z = v1.world_pos * inv_z1;
        let wpos2_z = v2.world_pos * inv_z2;

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
                    let interp_inv_z = inv_z0 * w0 + inv_z1 * w1 + inv_z2 * w2;
                    let z = 1.0 / interp_inv_z;
                    let normal = ((n0_z * w0 + n1_z * w1 + n2_z * w2) * z).normalize();

                    let col = (col0_z * w0 + col1_z * w1 + col2_z * w2) * z;
                    let world_pos = (wpos0_z * w0 + wpos1_z * w1 + wpos2_z * w2) * z;

                    let l = light_dir.normalize();

                    let v = (pos - world_pos).normalize();
                    let h = (l + v).normalize();

                    let ambient = 0.1;
                    let diffuse = normal.dot(l).max(0.0);
                    let spec = normal.dot(h).max(0.0).powf(16.0);

                    let intensity = (ambient + diffuse + spec).min(1.0);

                    fb.set_pixel(IVec2::new(x, y), z, (col * intensity).to_u32());
                }
            }
        }
    }
}
