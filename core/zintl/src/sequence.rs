use std::{
    any::*,
    mem::{Layout, alloc, dealloc},
};

pub struct Sequence {
    type_id: TypeId,
    size: usize,
    align: usize,
    capacity: usize,
    seq: *const u8,
}

impl Sequence {
    pub fn new<T: 'static>() -> Self {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        Sequence {
            type_id: TypeId::of::<T>(),
            size,
            align,
            capacity: 0,
            seq: std::ptr::null(),
        }
    }

    fn scale(&mut self) {
        let cap = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };

        let layout = Layout::from_size_align(self.size * self.capacity, self.align);
    }

    pub fn insert<T: 'static>(&mut self) -> usize {
        self.
    }

    pub fn get<R: 'static>(&mut self, idx: usize) -> Option<&mut R> {
        if TypeId::of::<R>() == self.type_id {
            let t = match self.seq.get(idx) {
                Some(b) => b,
                None => return None,
            };
            // SAFETY: A type of the input value is checked.
            let r = unsafe { std::mem::transmute(&*t) };
            Some(r)
        } else {
            None
        }
    }
}

pub struct Arena {
    seqs: Vec<Sequence>,
}
