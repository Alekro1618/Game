use std::marker::PhantomData;

pub trait SparseArrayIndex {
    fn index(&self) -> usize; 
}

impl<N : Into<usize> + Copy> SparseArrayIndex for N {
    fn index(&self) -> usize {
        (*self).into()
    }
}

#[derive(Debug)]
pub struct SparseArray<T, I = usize> {
    values : Vec<Option<T>>,
    marker : PhantomData<I>
}

impl<I, T>  Default for SparseArray<T, I> {
    fn default() -> Self {
        SparseArray::new()
    }
}

impl <I, T> SparseArray<T, I> {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    #[inline]
    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            marker: PhantomData
        }
    }
}

impl <I: SparseArrayIndex, T> SparseArray<T, I>  {
    #[inline]
    pub fn insert(&mut self, index: I, value: T) {
        let index = index.index();
        if self.values.len() <= index {
            self.values.resize_with(index + 1, || None);
        }
        self.values[index] = Some(value);
    }

    #[inline]
    pub fn contains(&self, index: I) -> bool {
        self.values.get(index.index()).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.values.get(index.index()).and_then(Option::as_ref)
    }
}