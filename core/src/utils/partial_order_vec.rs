use engine::utils::SparseArray;


#[derive(Debug, Clone)]
pub struct PartialOrderVec<T> {
    elements : SparseArray<T, LeafId>,
    tree : SparseArray<Next, LeafId>,
}

#[derive(Debug, Clone)]
pub enum Next {
    Id(LeafId),
    Multiple(Vec<LeafId>)
}

#[derive(Debug, Clone, Copy)]
pub struct LeafId(u32);

impl Into<usize> for LeafId {
    fn into(self) -> usize {
        self.0 as usize
    }
}


impl<T> PartialOrderVec<T> {
    pub fn new() -> Self {
        Self { 
            elements: SparseArray::new(), 
            tree: SparseArray::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::partial_order_vec::PartialOrderVec;

    #[test]
    pub fn example() {

    }
}