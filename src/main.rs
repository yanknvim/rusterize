mod framebuffer;
mod rasterizer;
mod vec;

use crate::{
    framebuffer::FrameBuffer,
    rasterizer::Rasterizer,
    vec::{Vec2, Vec4},
};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    let mut ras = Rasterizer::new(&mut fb);

    let mut window = Window::new("Rusterize", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(60);

    let p0 = Vec2::new(0.0, 0.5);
    let p1 = Vec2::new(0.5, -0.5);
    let p2 = Vec2::new(-0.5, -0.5);

    let mut t = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ras.clear(Vec4::splat(0.0));
        
        ras.draw_triangle(p0, p1, p2, Vec4::new(1.0, 0.0, 0.0, 1.0));
        window.update_with_buffer(ras.buf(), WIDTH, HEIGHT).unwrap();

        t += 0.01;
    }
}
