use std::ops;
use crate::math::*;
use crate::circuit::*;




// provide descriptive types for voxel data
pub enum Type {
    Empty,
    OutBound, // out of bound error
    Solid,
    Wire(Channel),
    Const,
    Gate(Operator),
    Mux,
    Demux,
    Keyboard,
}

pub struct Model {
    pub size: Vec3i,
    pub buffer: Vec<Type>,
    out_off_bound: Type,
}


impl Model {

    // prepare a model but its buffer is still empty
    pub fn prepare(size: Vec3i) -> Self {
        let buffer = Vec::<Type>::with_capacity(size.index_range());
        Self {
            size: size,
            buffer: buffer,
            out_off_bound: Type::OutBound,
        }
    }

    #[inline(always)]
    fn check_bounds(&self, i: &Vec3i) -> bool {
        i.x < self.size.x && i.y < self.size.y && i.z < self.size.z
    }

    #[inline(always)]
    fn internal_index(&self, i: &Vec3i) -> usize {
        i.x + i.y * self.size.x + i.z * self.size.y
    }
}


impl ops::Index<Vec3i> for Model {
    type Output = Type;

    #[inline]
    fn index<'a>(&'a self, i: Vec3i) -> &'a Type {
        if self.check_bounds(&i) {
            let index = self.internal_index(&i);
            &self.buffer[index]
        } else {
            &Type::OutBound
        }
    }
}

impl ops::IndexMut<Vec3i> for Model {

    #[inline]
    fn index_mut<'a>(&'a mut self, i: Vec3i) -> &'a mut Type {
        if self.check_bounds(&i) {
            let index = self.internal_index(&i);
            &mut self.buffer[index]
        } else {
            &mut self.out_off_bound
        }
    }
}