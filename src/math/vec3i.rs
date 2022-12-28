/**
 * represent a simple integer 3D vector
 */
use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};
use std::{ops, cmp};


pub enum Axis {X, Y, Z}



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

    #[inline]
    pub fn as_vec3(&self) -> Vec3 {
        Vec3 { 
            x: self.x as f32, 
            y: self.y as f32, 
            z: self.z as f32,
        }
    }

    #[inline]
    pub fn from_vec3(o: Vec3) -> Self {
        Self {
            x: o.x as usize,
            y: o.y as usize,
            z: o.z as usize,
        }
    }

    // return the index of the axis in decreasing order
    #[inline]
    pub fn order_axis(&self) -> [Axis; 3] {
        let mut mask: usize = 0b000;
        if self.x > self.y {mask |= 0b001;}
        if self.y > self.z {mask |= 0b010;}
        if self.z > self.x {mask |= 0b100;}

        match mask {
            0b011 => [Axis::X, Axis::Y, Axis::Z],
            0b001 => [Axis::X, Axis::Z, Axis::Y],
            0b010 => [Axis::Y, Axis::X, Axis::Z],
            0b110 => [Axis::Y, Axis::Z, Axis::X],
            0b101 => [Axis::Z, Axis::X, Axis::Y],
            0b100 => [Axis::Z, Axis::Y, Axis::X],
            _     => [Axis::X, Axis::Y, Axis::Z]
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


