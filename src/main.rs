mod framebuffer;
mod obj;
mod rasterizer;
mod scene;
mod vec;

use crate::{
    framebuffer::FrameBuffer,
    rasterizer::{Rasterizer, Vertex},
    scene::project,
    vec::{Vec2, Vec3, Vec4},
};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let obj = obj::load_obj("./src/teapot.obj");

    let mut fb = FrameBuffer::new(WIDTH, HEIGHT);
    let mut ras = Rasterizer::new();

    let mut window = Window::new("Rusterize", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let light_dir = Vec3::new(1.0, 1.0, -1.0).normalize();
    let pos = Vec3::splat(0.0);

    window.set_target_fps(60);

    let mut t = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ras.clear(&mut fb, Vec4::splat(0.0));

        for face in &obj.indices {
            let offset = Vec3::new(0.0, -1.5, 5.0);
            let v0 = obj.vertices[face[0]].rotate_y(t) + offset;
            let v1 = obj.vertices[face[1]].rotate_y(t) + offset;
            let v2 = obj.vertices[face[2]].rotate_y(t) + offset;

            let normal = (v1 - v0).cross(v2 - v0).normalize();

            if v0.z <= 0.1 || v1.z <= 0.1 || v2.z <= 0.1 {
                continue;
            }

            let l = light_dir.normalize();

            let v = (v0-pos).normalize();
            let h = (l + v).normalize();

            let ambient = 0.1;
            let diffuse = normal.dot(l).max(0.0);
            let spec = normal.dot(h).max(0.0).powf(16.0);

            let intensity = (ambient + diffuse + spec).min(1.0);

            let col = Vec4::new(intensity, intensity, intensity, 1.0);

            let vt0 = Vertex {
                pos: project(v0),
                z: v0.z,
                col,
            };

            let vt1 = Vertex {
                pos: project(v1),
                z: v1.z,
                col,
            };

            let vt2 = Vertex {
                pos: project(v2),
                z: v2.z,
                col,
            };

            if normal.dot(v0) < 0.0 {
                ras.draw_triangle(&mut fb, vt0, vt1, vt2);
            }
        }

        window.update_with_buffer(&fb.buf, WIDTH, HEIGHT).unwrap();

        t += 0.01;
    }
}
