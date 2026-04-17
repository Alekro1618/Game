use std::{alloc::{Layout, alloc, realloc}, any::Any, cell::UnsafeCell, fmt::Alignment, num::NonZeroUsize, ptr::NonNull};

use crate::ecs::{change_detection::tick::Tick, components::Component, entities::id::EntityId};

pub struct Column {
    item_layout: Layout,
    data: NonNull<u8>,
    drop: Option<unsafe fn(*mut u8)>,
    len : usize,
    capacity: usize,
    changed_ticks : Vec<Tick>,
    added_ticks : Vec<Tick>
}

impl Column {
    pub fn new(layout : Layout, drop : Option<unsafe fn(*mut u8)>) -> Self {
        Self { 
            item_layout: layout,
            data: NonNull::dangling(),
            drop: drop,
            len: 0,
            capacity: 0,
            changed_ticks: Vec::new(),
            added_ticks: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize, layout : Layout, drop : Option<unsafe fn(*mut u8)>) -> Self {
        if capacity == 0 {
            return Self::new(layout, drop);
        }

        let bytes = layout.size() * capacity;
        let alloc_layout = Layout::from_size_align(bytes, layout.align()).unwrap();
        let ptr = unsafe { alloc(alloc_layout)};
        Self {
            item_layout: layout,
            data: NonNull::new(ptr).unwrap(),
            drop: drop,
            len: 0,
            capacity: capacity,
            added_ticks: Vec::with_capacity(capacity),
            changed_ticks: Vec::with_capacity(capacity),
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data.as_ptr()
    }

    unsafe fn row_ptr(&self, row: usize) -> *mut u8 {
        self.data.as_ptr().add(row * self.item_layout.size())
    }


    fn grow(&mut self) {
        let new_cap = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        if self.item_layout.size() == 0 {
            self.capacity = new_cap;
            return;
        }
        let old_layout = Layout::from_size_align(
            self.item_layout.size() * self.capacity,
            self.item_layout.align(),
        ).unwrap();
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

    pub fn push<T : Component>(&mut self, data : T, tick : Tick) {
        if self.len == self.capacity {
            self.grow();
        }
        unsafe {
            let ptr = self.row_ptr(self.len).cast::<T>();
            ptr.write(data)
        }
        self.len += 1;
        self.changed_ticks.push(tick);
        self.added_ticks.push(tick);
    }

    pub unsafe fn get_slice<T: Component>(&self) -> &[UnsafeCell<T>] {
        unsafe  {
            std::slice::from_raw_parts(
            self.data.as_ptr() as *const UnsafeCell<T>,
                self.len,
            )
        }
    }


    pub fn get<T : Component>(&self, row : usize) -> Option<&T> {
        if row >= self.len { return None; }
        unsafe { Some(&*self.row_ptr(row).cast::<T>()) }
    }

    pub fn get_mut<T : Component>(&mut self, row : usize) -> Option<&mut T> {
        if row >= self.len { return None; }
        unsafe { Some(&mut *self.row_ptr(row).cast::<T>()) }
    }

    pub fn swap_remove(&mut self, row : usize) {
        let last = self.len - 1;
        if row != last {
            unsafe {
                // drop the element at `row`
                (self.drop.unwrap())(self.row_ptr(row));
                // move last element into the gap
                let src = self.row_ptr(last);
                let dst = self.row_ptr(row);
                std::ptr::copy_nonoverlapping(src, dst, self.item_layout.size());
            }
        } else {
            unsafe { (self.drop.unwrap())(self.row_ptr(row)); }
        }
        self.len -= 1;
        self.changed_ticks.swap_remove(row);
        self.added_ticks.swap_remove(row);
    }

    pub fn len(&self) -> usize{
        self.len
    }
}


impl Drop for Column {
    fn drop(&mut self) {
        if let Some(drop_fn) = self.drop {
            for i in 0..self.len {
                unsafe { drop_fn(self.row_ptr(i)); }
            }
        }
        if self.capacity > 0 && self.item_layout.size() > 0 {
            let layout = Layout::from_size_align(
                self.item_layout.size() * self.capacity,
                self.item_layout.align(),
            ).unwrap();
            unsafe { std::alloc::dealloc(self.data.as_ptr(), layout); }
        }
    }
}
