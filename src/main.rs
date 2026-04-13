mod framebuffer;
mod rasterizer;
mod vec;

use crate::{
    framebuffer::FrameBuffer,
    rasterizer::{Rasterizer, Vertex},
    vec::{Vec2, Vec4},
};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    let mut ras = Rasterizer::new();

    let mut window = Window::new("Rusterize", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(60);

    let mut t = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ras.clear(&mut fb, Vec4::splat(0.0));

        let v0 = Vertex {
            pos: Vec2::new(0.0, 0.5),
            z: 0.3,
            col: Vec4::new(1.0, 0.0, 0.0, 1.0),
        };

        let v1 = Vertex {
            pos: Vec2::new(-0.5, -0.5),
            z: 0.3,
            col: Vec4::new(1.0, 0.0, 0.0, 1.0),
        };

        let v2 = Vertex {
            pos: Vec2::new(0.5, -0.5),
            z: 0.3,
            col: Vec4::new(1.0, 0.0, 0.0, 1.0),
        };

        let v3 = Vertex {
            pos: Vec2::new(0.0, -0.5),
            z: 0.8,
            col: Vec4::new(0.0, 0.0, 1.0, 1.0),
        };

        let v4 = Vertex {
            pos: Vec2::new(-0.5, 0.5),
            z: 0.8,
            col: Vec4::new(0.0, 0.0, 1.0, 1.0),
        };

        let v5 = Vertex {
            pos: Vec2::new(0.5, 0.5),
            z: 0.8,
            col: Vec4::new(0.0, 0.0, 1.0, 1.0),
        };

        ras.draw_triangle(&mut fb, v0, v1, v2);
        ras.draw_triangle(&mut fb, v3, v4, v5);
        window.update_with_buffer(&fb.buf, WIDTH, HEIGHT).unwrap();

        t += 0.01;
    }
}
