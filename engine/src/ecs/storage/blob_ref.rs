use std::{alloc::{Layout, alloc, realloc}, cell::UnsafeCell, fmt::Display, marker::PhantomData, ptr::NonNull};


#[derive(Clone, Copy)]
pub struct BlobRef<'a> {
    item_layout: Layout,
    data: *mut u8,
    drop: Option<unsafe fn(*mut u8)>,
    _lifetime : PhantomData<&'a ()>
}


impl<'a> BlobRef<'a> {
    pub fn layout(&self) -> Layout {
        self.item_layout
    }

    pub fn new(ptr : *mut u8, layout : Layout, drop : Option<unsafe fn(*mut u8)>) -> Self {
        let bytes = layout.size();
        let alloc_layout = Layout::from_size_align(bytes, layout.align()).unwrap();
        let ptr = unsafe { alloc(alloc_layout)};

        Self { 
            item_layout: layout,
            data: ptr,
            drop: drop,
            _lifetime: PhantomData
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }

    pub unsafe fn set_unchecked<T>(&mut self, data : T) {
        unsafe {
            let dst = self.as_mut_ptr();
            core::ptr::write(dst.cast::<T>(), data);
        }
    }

    pub unsafe fn get_unchecked<T>(self) -> &'a T {
        unsafe { &*self.as_ptr().cast::<T>() }
    }

    pub unsafe fn get_mut_unchecked<T>(&mut self) -> &mut T {
        unsafe { &mut *self.as_mut_ptr().cast::<T>() }
    }

    pub unsafe fn write(&mut self, data : u8) {
        unsafe {
            let dst = self.as_mut_ptr();
            core::ptr::write(dst, data);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::alloc::Layout;
    use crate::ecs::storage::{blob_ref::BlobRef};

    #[derive(PartialEq, Eq, Debug)]
    pub struct D(Vec<u32>);

    // #[test]
    // fn set_unchecked() {
    //     let layout = Layout::new::<u32>();
    //     let mut blob = BlobRef::new(layout, None);
        
    //     let a = 1u32;
    //     unsafe {
    //         blob.set_unchecked::<u32>(a);
            
    //     }
    // }

    // pub struct Test;

    // #[test]
    // fn test_replace_get_unchecked() {
    //     let layout = Layout::new::<u32>();
    //     let mut blob = BlobRef::new(layout, None);
        
    //     let a = 1u32;
    //     let b = 2u32;
    //     unsafe  {
    //         blob.set_unchecked::<u32>(a);
    //         assert_eq!(blob.get_unchecked::<u32>(), &1u32);
    //         blob.set_unchecked::<u32>(b);
    //         assert_eq!(blob.get_unchecked::<u32>(), &3u32);
    //     }
    // }
}