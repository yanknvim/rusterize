use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_i32(self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();

        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn splat(x: f32) -> Self {
        Self {
            x: x,
            y: x,
            z: x,
            w: x,
        }
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        let x = (self.x.clamp(0.0, 1.0) * 255.0) as u32;
        let y = (self.y.clamp(0.0, 1.0) * 255.0) as u32;
        let z = (self.z.clamp(0.0, 1.0) * 255.0) as u32;
        let w = (self.w.clamp(0.0, 1.0) * 255.0) as u32;

        (w << 24) | (x << 16) | (y << 8) | z
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
