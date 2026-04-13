mod framebuffer;
mod vec;

use crate::{framebuffer::FrameBuffer, vec::Vec4};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    fb.clear(0);

    let mut window = Window::new(
        "Rusterize",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let fx = x as f32 / WIDTH as f32;
                let fy = y as f32 / HEIGHT as f32;
                let color = Vec4::new(fx, fy, 1.0 - fx, 1.0);
                fb.set_pixel(x, y, color.to_u32());
            }
        }

        window.update_with_buffer(fb.buf(), WIDTH, HEIGHT).unwrap()
    }
}
