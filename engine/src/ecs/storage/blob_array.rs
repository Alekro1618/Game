use std::{
    alloc::{Layout, alloc, realloc},
    cell::UnsafeCell,
    ptr::NonNull,
};

use crate::ecs::storage::blob_ref::BlobRef;

pub struct BlobArray {
    item_layout: Layout,
    data: NonNull<u8>,
    capacity: usize,
    len: usize,
    drop: Option<unsafe fn(*mut u8)>,
}

impl BlobArray {
    pub fn layout(&self) -> Layout {
        self.item_layout
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn new(layout: Layout, drop: Option<unsafe fn(*mut u8)>) -> Self {
        Self {
            item_layout: layout,
            data: NonNull::dangling(),
            len: 0,
            capacity: 0,
            drop: drop,
        }
    }

    pub fn with_capacity(
        capacity: usize,
        layout: Layout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Self {
        if capacity == 0 {
            return Self::new(layout, drop);
        }

        let bytes = layout.size() * capacity;
        let alloc_layout = Layout::from_size_align(bytes, layout.align()).unwrap();
        let ptr = unsafe { alloc(alloc_layout) };

        Self {
            item_layout: layout,
            len: 0,
            data: NonNull::new(ptr).unwrap(),
            capacity: capacity,
            drop: drop,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data.as_ptr()
    }

    unsafe fn row_ptr(&self, row: usize) -> *mut u8 {
        unsafe { self.data.as_ptr().add(row * self.item_layout.size()) }
    }

    fn grow(&mut self) {
        let new_cap = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        if self.item_layout.size() == 0 {
            self.capacity = new_cap;
            return;
        }
        let old_layout = Layout::from_size_align(
            self.item_layout.size() * self.capacity,
            self.item_layout.align(),
        )
        .unwrap();

        let new_size = self.item_layout.size() * new_cap;
        let ptr = if self.capacity == 0 {
            let new_layout = Layout::from_size_align(new_size, self.item_layout.align()).unwrap();
            unsafe { alloc(new_layout) }
        } else {
            unsafe { realloc(self.data.as_ptr(), old_layout, new_size) }
        };

        self.data = NonNull::new(ptr).expect("allocation failed");
        self.capacity = new_cap;
    }

    pub unsafe fn push_unchecked<T>(&mut self, data: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            let dst = self.row_ptr(self.len);
            core::ptr::write(dst.cast::<T>(), data);
            self.len += 1;
        }
    }

    pub unsafe fn get_blob_ref<'a>(&'a self, row: usize) -> BlobRef<'a> {
        unsafe {
            let src = self.row_ptr(row);

            BlobRef::new(src, self.item_layout, self.drop)
        }
    }

    #[inline]
    pub unsafe fn get_unchecked<T>(&self, row: usize) -> &T {
        unsafe { &*self.row_ptr(row).cast::<T>() }
    }

    #[inline]
    pub unsafe fn get_mut_unchecked<T>(&mut self, row: usize) -> &mut T {
        unsafe { &mut *self.row_ptr(row).cast::<T>() }
    }

    #[inline]
    /// Safety: Row should be in bounds and raw pointer to appropriate type.
    /// Caller should use it only if the existing row is not null
    pub unsafe fn replace_unchecked<T>(&mut self, data: T, row: usize) {
        unsafe {
            let dst = self.row_ptr(row);
            core::ptr::write(dst.cast::<T>(), data);
        }
    }

    #[inline]
    /// Safety: Row should be in bounds and raw pointer to appropriate type.
    /// Caller should use it only if the existing row is not null
    pub unsafe fn move_to_unchecked(&mut self, dst: &mut Self, row: usize, last_index: usize) {
        unsafe {
            if row != last_index {
                core::ptr::swap_nonoverlapping(
                    self.row_ptr(row),
                    self.row_ptr(last_index),
                    self.item_layout.size(),
                );
            }

            self.len -= 1;

            if dst.len == dst.capacity {
                dst.grow();
            }

            core::ptr::swap_nonoverlapping(
                self.row_ptr(last_index),
                dst.row_ptr(dst.len),
                self.item_layout.size(),
            );

            dst.len += 1;
        }
    }

    /// SAFETY: the inner data will remain valid for as long as 'self.
    pub unsafe fn get_sub_slice<T>(&self, slice_len: usize) -> &[UnsafeCell<T>] {
        assert!(self.capacity >= slice_len);
        unsafe {
            core::slice::from_raw_parts(self.data.as_ptr() as *const UnsafeCell<T>, slice_len)
        }
    }

    #[inline]
    /// Safety
    pub unsafe fn swap_remove_unchecked(&mut self, row_to_remove: usize, row_to_keep: usize) {
        unsafe {
            if row_to_remove != row_to_keep {
                self.swap_remove_unchecked_nonoverlapping(row_to_remove, row_to_keep);
                return;
            }
            self.len -= 1;
        }
    }

    #[inline]
    pub unsafe fn swap_remove_unchecked_nonoverlapping(
        &mut self,
        row_to_remove: usize,
        row_to_keep: usize,
    ) {
        unsafe {
            core::ptr::swap_nonoverlapping(
                self.row_ptr(row_to_keep),
                self.row_ptr(row_to_remove),
                self.item_layout.size(),
            );

            self.len -= 1;
        }
    }
}

impl Drop for BlobArray {
    fn drop(&mut self) {
        if let Some(drop_fn) = self.drop {
            for i in 0..self.len {
                unsafe {
                    drop_fn(self.row_ptr(i));
                }
            }
        }
        if self.capacity > 0 && self.item_layout.size() > 0 {
            let layout = Layout::from_size_align(
                self.item_layout.size() * self.capacity,
                self.item_layout.align(),
            )
            .unwrap();
            unsafe {
                std::alloc::dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::Layout;

    use crate::ecs::storage::blob_array::BlobArray;

    #[derive(PartialEq, Eq, Debug)]
    pub struct D(Vec<u32>);

    #[test]
    fn test_push_get_unchecked() {
        let layout = Layout::new::<u32>();
        let mut array = BlobArray::new(layout, None);

        let a = 1u32;
        let b = 2u32;
        unsafe {
            array.push_unchecked::<u32>(a);
            array.push_unchecked::<u32>(b);
            assert_eq!(array.get_unchecked::<u32>(0), &1u32);
            assert_eq!(array.get_unchecked::<u32>(1), &2u32);
        }
    }

    pub struct Test;

    #[test]
    fn test_sub_slice() {
        let layout = Layout::new::<Test>();
        let mut array = BlobArray::new(layout, None);

        let a = Test;
        let b = Test;
        let c = Test;
        unsafe {
            array.push_unchecked(a);
            array.push_unchecked(b);
            array.push_unchecked(c);

            let d = array.get_sub_slice::<Test>(3);
            assert_eq!(d.len(), 3);
        }
    }

    #[test]
    fn test_replace_get_unchecked() {
        let layout = Layout::new::<u32>();
        let mut array = BlobArray::new(layout, None);

        let a = 1u32;
        let b = 2u32;
        let c = 3u32;
        unsafe {
            array.push_unchecked(a);
            array.push_unchecked(b);
            assert_eq!(array.get_unchecked::<u32>(0), &1u32);
            array.replace_unchecked(c, 0);
            assert_eq!(array.get_unchecked::<u32>(0), &3u32);
        }
    }

    #[test]
    fn test_swap_remove_unchecked() {
        let layout = Layout::new::<u32>();
        let mut array = BlobArray::new(layout, None);

        let a = 1u32;
        let b = 2u32;
        let c = 3u32;
        let d = 4u32;
        unsafe {
            array.push_unchecked(a);
            array.push_unchecked(b);
            array.push_unchecked(c);
            assert_eq!(array.get_unchecked::<u32>(0), &1u32);
            array.swap_remove_unchecked(0, 2);
            assert_eq!(array.get_unchecked::<u32>(0), &3u32);
            assert_eq!(array.get_unchecked::<u32>(1), &2u32);
            array.push_unchecked(d);
            assert_eq!(array.get_unchecked::<u32>(2), &4u32);
        }

        let mut array = BlobArray::new(layout, None);

        let a = 1u32;
        let b = 2u32;
        unsafe {
            array.push_unchecked(a);
            assert_eq!(array.get_unchecked::<u32>(0), &1u32);
            array.swap_remove_unchecked(0, 0);
            array.push_unchecked(b);
            assert_eq!(array.get_unchecked::<u32>(0), &2u32);
        }
    }

    #[test]
    fn test_heap_structs() {
        let layout = Layout::new::<D>();
        let mut array = BlobArray::new(layout, None);

        let a = D(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let b = D(vec![9, 8, 7, 6, 5]);
        unsafe {
            array.push_unchecked(a);
            array.push_unchecked(b);

            let d = array.get_mut_unchecked::<D>(0);
            d.0.push(10);

            let d = array.get_mut_unchecked::<D>(1);
            d.0.remove(0);

            assert_eq!(
                array.get_unchecked::<D>(0),
                &D(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
            );
            assert_eq!(array.get_unchecked::<D>(1), &D(vec![8, 7, 6, 5]));
        }
    }

    #[test]
    fn test_ownership() {
        let layout = Layout::new::<u32>();
        let mut array = BlobArray::new(layout, None);

        let a = 1u32;
        {
            let b = 2u32;
            unsafe {
                array.push_unchecked(a);
                array.push_unchecked(b);
            }
        }

        unsafe {
            assert_eq!(array.get_unchecked::<u32>(0), &1u32);
            assert_eq!(array.get_unchecked::<u32>(1), &2u32);
        }
    }

    #[test]
    fn test_move_to_unchecked() {
        let layout = Layout::new::<u32>();
        let mut array1 = BlobArray::new(layout, None);
        let mut array2 = BlobArray::new(layout, None);

        let a = 1u32;
        let b = 2u32;
        let c = 3u32;

        unsafe {
            array1.push_unchecked(a);
            array1.push_unchecked(b);
            array2.push_unchecked(c);

            array1.move_to_unchecked(&mut array2, 0, array1.len - 1);

            assert_eq!(array1.get_unchecked::<u32>(0), &2u32);
            assert_eq!(array2.get_unchecked::<u32>(0), &3u32);
            assert_eq!(array2.get_unchecked::<u32>(1), &1u32);
        }
    }
}
