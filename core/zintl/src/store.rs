use std::{any::TypeId, marker::PhantomData};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoreId {
    pub(crate) seq: usize,
    pub(crate) idx: usize,
}

impl StoreId {
    pub const UNINITIALIZED: Self = StoreId { seq: 0, idx: 0 };
}

pub struct Store<T: 'static> {
    id: StoreId,
    phantom: PhantomData<T>,
}

impl<T: 'static> Store<T> {
    pub fn new() -> Self {
        Store {
            id: StoreId::UNINITIALIZED,
            phantom: PhantomData,
        }
    }

    pub(crate) fn init(&mut self, id: StoreId) {
        self.id = id;
    }

    pub(crate) fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
