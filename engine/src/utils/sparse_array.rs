use std::{marker::PhantomData, usize};

pub trait SparseArrayIndex {
    fn index(&self) -> usize;
}

impl<N : Into<usize> + Copy> SparseArrayIndex for N {
    fn index(&self) -> usize {
        (*self).into()
    }
}

#[derive(Debug, Clone)]
pub struct  SparseArray<T, I = usize> {
    values : Vec<Option<T>>,
    marker : PhantomData<I>,
    len : usize,
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
            marker: PhantomData,
            len : 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl <I: SparseArrayIndex, T> SparseArray<T, I>  {
    #[inline]
    pub fn insert(&mut self, index: I, value: T) {
        let index = index.index();
        if self.values.len() <= index {
            self.values.resize_with(index + 1, || None);
        }

        if self.values[index].is_none() {
            self.len += 1;
        }

        self.values[index] = Some(value);
    }

    #[inline]
    pub fn remove(&mut self, index: I) -> Option<T> {
        let index = index.index();
        if self.values.len() == 0 {
            return None;
        }

        if self.values[index].is_some() {
            self.len -= 1;
        }

        std::mem::replace(&mut self.values[index], None)
    }

    #[inline]
    pub fn contains(&self, index: I) -> bool {
        self.values.get(index.index()).is_some_and(Option::is_some)
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.values.get(index.index()).and_then(Option::as_ref)
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.values.get_mut(index.index()).and_then(Option::as_mut)
    }

}

impl <I: SparseArrayIndex + From<usize>, T> SparseArray<T, I> {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (I, &T)>{
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, f)| f.as_ref().and_then(|d| Some((I::from(i), d))))
            
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (I, &mut T)>{
        self.values
            .iter_mut()
            .enumerate()
            .filter_map(|(i, f)| f.as_mut().and_then(|d| Some((I::from(i), d))))
            
    }
}