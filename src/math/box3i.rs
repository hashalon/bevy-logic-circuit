/**
 * represent a model to build and to display in bevy
 */
use crate::math::vec3i::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Box3i {
    pub begin : Vec3i,
    pub end   : Vec3i,
}


impl Box3i {
    #[inline]
    pub const fn new (begin: Vec3i, end: Vec3i) -> Self {
        Self { begin: begin, end: end }
    }

    #[inline]
    pub const fn define (x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2: usize) -> Self {
        let begin = Vec3i::new(x1, y1, z1);
        let end   = Vec3i::new(x2, y2, z2);
        Self { begin: begin, end: end }
    }

    #[inline]
    pub fn size(&self) -> Vec3i {
        self.end - self.begin
    }

    #[inline]
    pub const fn inside(&self, point: Vec3i) -> bool {
        self.begin.lesser_equal(point) && point.lesser(self.end)
    }

    #[inline]
    pub const fn overlap(&self, other: Box3i) -> bool {
        self .begin.lesser_equal(other.end) && other.begin.lesser(self .end) ||
        other.begin.lesser_equal(self .end) && self .begin.lesser(other.end)
    }
}