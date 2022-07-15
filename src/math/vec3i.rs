/**
 * represent a simple integer 3D vector
 */
use serde::{Deserialize, Serialize};
use std::{ops, cmp::{min, max}};

pub const NB_COORDS: usize = 3;
pub type Vertex = [f32; NB_COORDS];

#[derive(Clone, Copy, Serialize, Deserialize, Hash)]
pub struct Vec3i {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3i {
    #[inline]
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x: x, y: y, z: z }
    }
    #[inline]
    pub const fn index_range(&self) -> usize {
        self.x * self.y * self.z
    }
    #[inline]
    pub const fn sum(&self) -> usize {
        self.x + self.y + self.z
    }
    #[inline]
    pub const fn to_vertex(&self) -> Vertex {
        [self.x as f32, self.y as f32, self.z as f32]
    }

    #[inline]
    pub const fn lesser(&self, o: Self) -> bool {
        self.x < o.x && self.y < o.y && self.z < o.z
    }
    #[inline]
    pub const fn greater(&self, o: Self) -> bool {
        self.x > o.x && self.y > o.y && self.z > o.z
    }
    #[inline]
    pub const fn lesser_equal(&self, o: Self) -> bool {
        self.x <= o.x && self.y <= o.y && self.z <= o.z
    }
    #[inline]
    pub const fn greater_equal(&self, o: Self) -> bool {
        self.x >= o.x && self.y >= o.y && self.z >= o.z
    }

    #[inline]
    pub fn min(&self, o: Self) -> Self {
        Self {
            x: min(self.x, o.x),
            y: min(self.y, o.y),
            z: min(self.z, o.z)
        }
    }
    #[inline]
    pub fn max(&self, o: Self) -> Self {
        Self {
            x: max(self.x, o.x),
            y: max(self.y, o.y),
            z: max(self.z, o.z)
        }
    }
}

impl ops::AddAssign for Vec3i {
    #[inline]
    fn add_assign(&mut self, o: Self) {
        self.x += o.x;
        self.y += o.y;
        self.z += o.z;
    }
}

impl ops::Add for Vec3i {
    type Output = Self;
    #[inline]
    fn add(self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl ops::SubAssign for Vec3i {
    #[inline]
    fn sub_assign(&mut self, o: Self) {
        self.x -= o.x;
        self.y -= o.y;
        self.z -= o.z;
    }
}

impl ops::Sub for Vec3i {
    type Output = Self;
    #[inline]
    fn sub(self, o: Self) -> Self {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl ops::MulAssign for Vec3i {
    #[inline]
    fn mul_assign(&mut self, o: Self) {
        self.x *= o.x;
        self.y *= o.y;
        self.z *= o.z;
    }
}

impl ops::Mul for Vec3i {
    type Output = Self;
    #[inline]
    fn mul(self, o: Self) -> Self {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl ops::DivAssign for Vec3i {
    #[inline]
    fn div_assign(&mut self, o: Self) {
        self.x /= o.x;
        self.y /= o.y;
        self.z /= o.z;
    }
}

impl ops::Div for Vec3i {
    type Output = Self;
    #[inline]
    fn div(self, o: Self) -> Self {
        Self {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        }
    }
}
