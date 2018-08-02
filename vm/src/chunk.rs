//! Chunks of memory allowing for Vec-like operations.
//!
//! A Chunk is a region of memory of a given type, with a fixed amount of
/// values. Chunks are optimized for performance, sacrificing safety in the
/// process.
///
/// Chunks do not drop the individual values. This means that code using a Chunk
/// must take care of this itself.
use std::alloc::{self, Layout};
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

pub struct Chunk<T> {
    ptr: *mut T,
    capacity: usize,
}

#[cfg_attr(feature = "cargo-clippy", allow(len_without_is_empty))]
impl<T> Chunk<T> {
    pub fn new(capacity: usize) -> Self {
        let layout = unsafe {
            Layout::from_size_align_unchecked(
                mem::size_of::<T>() * capacity,
                mem::align_of::<T>(),
            )
        };

        let ptr = unsafe { alloc::alloc(layout) as *mut T };

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        let mut chunk = Chunk { ptr, capacity };

        chunk.reset();
        chunk
    }

    pub fn len(&self) -> usize {
        self.capacity
    }

    pub fn reset(&mut self) {
        unsafe {
            // We need to zero out the memory as otherwise we might get random
            // garbage.
            ptr::write_bytes(self.ptr, 0, self.capacity);
        }
    }
}

impl<T> Index<usize> for Chunk<T> {
    type Output = T;

    fn index(&self, offset: usize) -> &T {
        unsafe { &*self.ptr.offset(offset as isize) }
    }
}

impl<T> IndexMut<usize> for Chunk<T> {
    fn index_mut(&mut self, offset: usize) -> &mut T {
        unsafe { &mut *self.ptr.offset(offset as isize) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use object_pointer::ObjectPointer;

    #[test]
    fn test_len() {
        let chunk = Chunk::<usize>::new(4);

        assert_eq!(chunk.len(), 4);
    }

    #[test]
    fn test_reset() {
        let mut chunk = Chunk::<ObjectPointer>::new(1);

        chunk[0] = ObjectPointer::integer(5);
        chunk.reset();

        assert!(chunk[0].is_null());
    }

    #[test]
    fn test_indexing_with_integer() {
        let mut chunk = Chunk::new(1);

        assert_eq!(chunk[0], 0);

        chunk[0] = 10;

        assert_eq!(chunk[0], 10);
    }

    #[test]
    fn test_indexing_with_object_pointer() {
        let mut chunk = Chunk::<ObjectPointer>::new(2);

        assert!(chunk[0].is_null());

        chunk[0] = ObjectPointer::integer(5);

        assert!(chunk[0] == ObjectPointer::integer(5));
        assert!(chunk[1].is_null());
    }
}
