use crate::vec::*;

pub fn project(v: Vec3) -> Vec2 {
    let fov = 1.0;

    let x = v.x / v.z * fov;
    let y = v.y / v.z * fov;

    Vec2 { x, y }
}
