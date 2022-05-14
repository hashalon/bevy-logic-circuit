use std::ops;
use crate::circuit::*;


//#[derive(Clone, Copy)]
pub struct Vec3i {
    x: usize,
    y: usize,
    z: usize,
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