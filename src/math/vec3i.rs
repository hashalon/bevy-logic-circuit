/**
 * represent a simple integer 3D vector
 */
use std::ops;


//#[derive(Clone, Copy)]
pub struct Vec3i {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Vec3i {

    #[inline]
    pub fn make(x: usize, y: usize, z: usize) -> Self {
        Self{
            x: x,
            y: y,
            z: z,
        }
    }

    #[inline]
    pub fn index_range(&self) -> usize {
        self.x * self.y * self.z
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
        Self{
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}