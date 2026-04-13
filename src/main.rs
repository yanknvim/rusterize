mod framebuffer;
mod vec;

use crate::{
    framebuffer::FrameBuffer,
    vec::{Vec2, Vec4},
};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Rusterize", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    window.set_target_fps(60);

    let p0 = Vec2::new(-1.0, -1.0);
    let p1 = Vec2::new(1.0, 1.0);

    let mut t = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        fb.clear(0);

        let xy0 = p0.rotate(t);
        let xy1 = p1.rotate(t);

        let x0 = ((xy0.x + 1.0) * 0.5 * WIDTH as f32) as i32;
        let y0 = ((1.0 - xy0.y) * 0.5 * HEIGHT as f32) as i32;
        let x1 = ((xy1.x + 1.0) * 0.5 * WIDTH as f32) as i32;
        let y1 = ((1.0 - xy1.y) * 0.5 * HEIGHT as f32) as i32;

        fb.draw_line(x0, y0, x1, y1, Vec4::new(1.0, 0.0, 0.0, 1.0).to_u32());

        window.update_with_buffer(fb.buf(), WIDTH, HEIGHT).unwrap();

        t += 0.01;
    }
}
