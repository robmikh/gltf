use serde::{Serialize, Serializer};

use crate::add_and_get_index;

pub struct StorageIndex<T: Sized + Default + Serialize>(pub usize, std::marker::PhantomData<T>);

// These are implemented manually as a workaround for rust-lang/rust#26925
impl<T: Sized + Default + Serialize> Copy for StorageIndex<T> {}
impl<T: Sized + Default + Serialize> Clone for StorageIndex<T> {
    fn clone(&self) -> Self {
        Self(self.0, std::marker::PhantomData)
    }
}
impl<T: Sized + Default + Serialize> std::fmt::Debug for StorageIndex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StorageIndex")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}
impl<T: Sized + Default + Serialize> Default for StorageIndex<T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<T: Sized + Default + Serialize> Serialize for StorageIndex<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0 as u64)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Storage<T: Sized + Default + Serialize> {
    items: Vec<T>,
}

impl<T: Sized + Default + Serialize> Storage<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn allocate(&mut self) -> StorageIndex<T> {
        let item = T::default();
        let index = add_and_get_index(&mut self.items, item);
        StorageIndex(index, std::marker::PhantomData)
    }

    pub fn allocate_with(&mut self, item: T) -> StorageIndex<T> {
        let index = add_and_get_index(&mut self.items, item);
        StorageIndex(index, std::marker::PhantomData)
    }

    pub fn update(&mut self, index: StorageIndex<T>, item: T) -> Option<()> {
        let item_slot = self.items.get_mut(index.0)?;
        *item_slot = item;
        Some(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Sized + Default + Serialize> Default for Storage<T> {
    fn default() -> Self {
        Self::new()
    }
}
