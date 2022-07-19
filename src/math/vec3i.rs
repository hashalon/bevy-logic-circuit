/**
 * represent a simple integer 3D vector
 */
use serde::{Deserialize, Serialize};
use std::{ops, cmp};


#[derive(Clone, Copy, Eq, Serialize, Deserialize, Hash)]
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
    pub fn min(&self, o: Self) -> Self {
        Self {
            x: cmp::min(self.x, o.x),
            y: cmp::min(self.y, o.y),
            z: cmp::min(self.z, o.z)
        }
    }
    #[inline]
    pub fn max(&self, o: Self) -> Self {
        Self {
            x: cmp::max(self.x, o.x),
            y: cmp::max(self.y, o.y),
            z: cmp::max(self.z, o.z)
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

impl PartialEq for Vec3i {
    #[inline]
    fn eq(&self, o: &Self) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z
    }
}


impl PartialOrd for Vec3i {
    #[inline]
    fn partial_cmp(&self, _o: &Self) -> Option<cmp::Ordering> {
        None
    }
    #[inline]
    fn lt(&self, o: &Self) -> bool {
        self.x < o.x && self.y < o.y && self.z < o.z
    }
    #[inline]
    fn le(&self, o: &Self) -> bool {
        self.x <= o.x && self.y <= o.y && self.z <= o.z
    }
    #[inline]
    fn gt(&self, o: &Self) -> bool {
        self.x > o.x && self.y > o.y && self.z > o.z
    }
    #[inline]
    fn ge(&self, o: &Self) -> bool {
        self.x >= o.x && self.y >= o.y && self.z >= o.z
    }
}