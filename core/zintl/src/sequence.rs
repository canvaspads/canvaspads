use crate::store::StoreId;
use std::any::*;
use std::cell::UnsafeCell;
use std::collections::HashMap;

pub struct Sequence<T: 'static> {
    type_id: TypeId,
    seq: Vec<UnsafeCell<T>>,
    next_idx: usize,
}

impl<T: 'static> Sequence<T> {
    fn new() -> Self {
        Sequence {
            type_id: TypeId::of::<T>(),
            seq: Vec::new(),
            next_idx: 0,
        }
    }

    fn get<'a>(&mut self, idx: usize) -> Option<&'a mut T> {
        let t = match self.seq.get(idx) {
            Some(b) => b,
            None => return None,
        };
        // SAFETY: A type of the input value is checked.
        let r = unsafe { &mut *(t.get()) };
        Some(r)
    }

    fn push(&mut self, item: T) -> usize {
        self.seq.push(UnsafeCell::new(item));
        let idx = self.next_idx;
        self.next_idx += 1;
        idx
    }
}

trait IntoSequence {
    fn get_type_id(&self) -> TypeId;
}

impl<T: 'static> IntoSequence for Sequence<T> {
    fn get_type_id(&self) -> TypeId {
        self.type_id
    }
}

pub struct Arena<'a> {
    seqs: Vec<Box<dyn IntoSequence>>,
    seqid_table: HashMap<TypeId, usize>,
    next_seqid: usize,
    phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> Arena<'a> {
    pub fn new() -> Self {
        Arena {
            seqs: vec![],
            seqid_table: HashMap::new(),
            next_seqid: 0,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn get<R: 'static>(&mut self, sid: StoreId) -> Option<&'a mut R> {
        if let Some(seq) = self.seqs.get_mut(sid.seq) {
            if seq.get_type_id() == TypeId::of::<R>() {
                let seq_fatptr: *mut dyn IntoSequence = &mut **seq;
                let seq_ptr = seq_fatptr as *mut Sequence<R>;
                // SAFETY: A pointer isn't possibly going to be null.
                let seq_ref = unsafe { &mut *seq_ptr };
                return seq_ref.get(sid.idx);
            }
        }
        None
    }

    pub fn insert_new<T: 'static>(&mut self, item: T) -> StoreId {
        let type_id = TypeId::of::<T>();
        if let Some(seq_id) = self.seqid_table.get(&type_id) {
            if let Some(seq) = self.seqs.get_mut(*seq_id) {
                let seq_fatptr: *mut dyn IntoSequence = &mut **seq;
                let seq_ptr = seq_fatptr as *mut Sequence<T>;
                // SAFETY: A pointer isn't possibly going to be null.
                let seq_ref = unsafe { &mut *seq_ptr };
                let idx = seq_ref.push(item);

                StoreId { seq: *seq_id, idx }
            } else {
                panic!("no seq");
            }
        } else {
            let mut new_seq: Sequence<T> = Sequence::new();
            let idx = new_seq.push(item);
            self.seqs.push(Box::new(new_seq));
            let new_seqid = self.next_seqid;
            self.seqid_table.insert(type_id, new_seqid);
            self.next_seqid += 1;

            StoreId {
                seq: new_seqid,
                idx,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn seq_arena() {
        let mut arena = Arena::new();
        let sid_str = arena.insert_new(String::from("string"));
        let sid_int = arena.insert_new(35_i32);
        let item = arena.get::<String>(sid_str).unwrap();
        assert_eq!(*item, String::from("string"));
        assert_eq!(arena.get::<String>(sid_int), None);
        let item = arena.get::<i32>(sid_int).unwrap();
        assert_eq!(*item, 35_i32);
    }
}
