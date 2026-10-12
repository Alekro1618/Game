use std::{
    alloc::Layout,
    cell::UnsafeCell,
};

use crate::ecs::{
    change_detection::tick::Tick,
    components::Component,
    storage::{blob_array::BlobArray, blob_ref::BlobRef},
};

pub struct Column {
    array: BlobArray,
    changed_ticks: Vec<UnsafeCell<Tick>>,
    added_ticks: Vec<UnsafeCell<Tick>>,
}

impl Column {
    pub fn new(layout: Layout, drop: Option<unsafe fn(*mut u8)>) -> Self {
        Self {
            array: BlobArray::new(layout, drop),
            changed_ticks: Vec::new(),
            added_ticks: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn with_capacity(
        capacity: usize,
        layout: Layout,
        drop: Option<unsafe fn(*mut u8)>,
    ) -> Self {
        Self {
            array: BlobArray::with_capacity(capacity, layout, drop),
            added_ticks: Vec::with_capacity(capacity),
            changed_ticks: Vec::with_capacity(capacity),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.array.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.array.as_mut_ptr()
    }

    ///Safety: Caller is responsible for using generic type apropriate for the column.
    pub unsafe fn push_unchecked<T: Component>(&mut self, data: T, tick: Tick) {
        unsafe {
            self.array.push_unchecked(data);
        }
        self.changed_ticks.push(UnsafeCell::new(tick));
        self.added_ticks.push(UnsafeCell::new(tick));
    }

    ///Safety: Caller is responsible for using generic type apropriate for the column.
    pub unsafe fn replace<T: Component>(&mut self, data: T, row: usize, tick: Tick) {
        unsafe {
            self.array.replace_unchecked(data, row);
        }
        self.changed_ticks.insert(row, UnsafeCell::new(tick));
        self.added_ticks.insert(row, UnsafeCell::new(tick));
    }

    pub unsafe fn move_to(&mut self, dst: &mut Self, row: usize, last_index: usize, tick: Tick) {
        unsafe {
            self.array
                .move_to_unchecked(&mut dst.array, row, last_index);
        }

        self.changed_ticks.swap_remove(row);
        self.added_ticks.swap_remove(row);
        dst.changed_ticks.push(UnsafeCell::new(tick));
        dst.added_ticks.push(UnsafeCell::new(tick));
    }

    /// Get a slice to the data stored in this [`Column`].
    ///
    /// # Safety
    /// - `T` must match the type of data that's stored in this [`Column`]
    /// - `len` must match the actual length of this column (number of elements stored)
    pub unsafe fn get_data_slice<T: Component>(&self, len: usize) -> &[UnsafeCell<T>] {
        // SAFETY: Upheld by caller
        unsafe { self.array.get_sub_slice(len) }
    }

    /// Get a slice to the changed [`ticks`](Tick) in this [`Column`].
    ///
    /// # Safety
    /// - `len` must match the actual length of this column (number of elements stored)
    pub fn get_changed_ticks_slice(&self, len: usize) -> &[UnsafeCell<Tick>] {
        &self.changed_ticks[..len] 
    }

    /// Get a slice to the changed [`ticks`](Tick) in this [`Column`].
    ///
    /// # Safety
    /// - `len` must match the actual length of this column (number of elements stored)
    pub fn get_added_ticks_slice(&self, len: usize) -> &[UnsafeCell<Tick>] {
        &self.added_ticks[..len] 
    }

    ///Safety:
    pub unsafe fn get(&self, row: usize) -> *const u8 {
        unsafe { self.array.get_unchecked(row) }
    }

    pub unsafe fn get_as_blob_ref<'a>(&'a self, row: usize) -> BlobRef<'a> {
        unsafe { self.array.get_blob_ref(row) }
    }

    ///Safety:
    pub unsafe fn get_mut(&mut self, row: usize) -> *mut u8 {
        unsafe { self.array.get_mut_unchecked(row) }
    }

    ///Safety:
    pub unsafe fn swap_remove(&mut self, last_element_index: usize, row: usize) {
        unsafe {
            self.array.swap_remove_unchecked(row, last_element_index);
        }

        self.changed_ticks.swap_remove(row);
        self.added_ticks.swap_remove(row);
    }
}
