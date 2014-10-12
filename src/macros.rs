

/// Simplify implementation of the StackVecArray trait.
macro_rules! impl_stack_vec_array_basic_methods(
    ($len:expr) => (
        #[inline]
        fn get(&self, idx: uint) -> &T { self[idx].as_ref().unwrap() }
        #[inline]
        fn get_mut(&mut self, idx: uint) -> &mut T { self[idx].as_mut().unwrap() }
        #[inline]
        fn set(&mut self, idx: uint, elem: T) { self[idx] = Some(elem); }
        #[inline]
        fn remove(&mut self, idx: uint) -> T { self[idx].take().unwrap() }
        #[inline]
        fn size(&self) -> uint { $len }
    )
)



