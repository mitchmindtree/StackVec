
#![crate_name = "stack_vec"]
#![deny(missing_doc)]
#![feature(macro_rules)]

//! A small library for a stack-based Vec.

#[macro_escape]
mod macros;

/// A stack-based vector for fast allocation.
pub struct StackVec<A> {
    len: uint,
    data: A,
}

impl<T, A: StackVecArray<T>> StackVec<A> {

    /// Create an empty StackVec.
    #[inline]
    pub fn new() -> StackVec<A> {
        StackVec { len: 0u, data: StackVecArray::new(), }
    }

    /// Push an element onto the end of the StackVec.
    #[inline]
    pub fn push(&mut self, elem: T) {
        let idx = self.len;
        self.len += 1;
        self.data.set(idx, elem);
    }

    /// Remove and return the final element.
    #[inline]
    pub fn pop(&mut self) -> T {
        let idx = self.len - 1u;
        self.len -= 1;
        self.data.remove(idx)
    }

    /// Return the number of occupied elems in the StackVec.
    #[inline]
    pub fn len(&self) -> uint { self.len }

    /// Return an iterator over the elements.
    #[inline]
    pub fn iter<'a>(&'a self) -> Items<'a, T, A> {
        Items { data: &self.data, len: self.len(), count: 0u }
    }

    /// Return an immutable reference to the value at the given index.
    #[inline]
    pub fn get(&self, idx: uint) -> &T { self.data.get(idx) }

    /// Return a mutable reference to the value at the given index.
    #[inline]
    pub fn get_mut(&mut self, idx: uint) -> &mut T { self.data.get_mut(idx) }

    /// Set the given index with the given element.
    #[inline]
    pub fn set(&mut self, idx: uint, elem: T) { self.data.set(idx, elem) }

    /// Remove an element from the given index and return it.
    #[inline]
    pub fn remove(&mut self, idx: uint) -> T { self.data.remove(idx) }

    /// Return the length of the DspBuffer.
    #[inline]
    pub fn size(&self) -> uint { self.data.size() }

}

/// A struct for iterating over StackVec's elements.
pub struct Items<'a, T, A: 'a> {
    data: &'a A,
    len: uint,
    count: uint,
}

impl<'a, T, A: StackVecArray<T> + 'a> Iterator<&'a T> for Items<'a, T, A> {
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.count < self.len {
            let count = self.count;
            self.count += 1;
            Some(self.data.get(count))
        } else { None }
    }
}

/*
/// A struct for iterating over StackVec's elements mutably.
pub struct MutItems<'a, T: 'a, A: 'a> {
    data: &'a mut A,
    len: uint,
    count: uint,
}

impl<'a, T: 'a, A: StackVecArray<T> + 'a> Iterator<&'a mut T> for MutItems<'a, T, A> {
    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.count < self.len {
            let count = self.count;
            self.count += 1;
            Some(self.data.get_mut(count))
        } else { None }
    }
}
*/


///// A struct for iterating over a StackVec's elements mutably.
//pub struct MutItems<'a, T: 'a> {
//    ptr: *mut T,
//    end: *mut T,
//    marker: marker::ContravariantLifetime<'a>,
//    marker2: marker::NoCopy,
//}


/// A trait to be implemented for all fixed-size arrays that
/// are power of 2 up to size 1024.
pub trait StackVecArray<T> {
    /// Constructor for a StackVecArray.
    fn new() -> Self;
    /// Return an immutable reference to the value at the given index.
    fn get(&self, idx: uint) -> &T;
    /// Return a mutable reference to the value at the given index.
    fn get_mut(&mut self, idx: uint) -> &mut T;
    /// Set the given index with the given element.
    fn set(&mut self, idx: uint, elem: T);
    /// Remove an element from the given index and return it.
    fn remove(&mut self, idx: uint) -> T;
    /// Return the length of the DspBuffer.
    fn size(&self) -> uint;
}

/*
impl<T, A: StackVecArray<T>> StackVecArray<T> for StackVec<A> {
    #[inline]
    fn new() -> StackVec<A> { StackVec::new() }
    #[inline]
    fn get(&self, idx: uint) -> &T { self.data.get(idx) }
    #[inline]
    fn get_mut(&mut self, idx: uint) -> &mut T { self.data.get_mut(idx) }
    #[inline]
    fn set(&mut self, idx: uint, elem: T) { self.data.set(idx, elem) }
    #[inline]
    fn remove(&mut self, idx: uint) -> T { self.data.remove(idx) }
    #[inline]
    fn size(&self) -> uint { self.data.size() }
}
*/

pub type N2<T> = [Option<T>, ..2];
pub type N4<T> = [Option<T>, ..4];
pub type N8<T> = [Option<T>, ..8];
pub type N16<T> = [Option<T>, ..16];
pub type N32<T> = [Option<T>, ..32];
pub type N64<T> = [Option<T>, ..64];
pub type N128<T> = [Option<T>, ..128];
pub type N256<T> = [Option<T>, ..256];
pub type N512<T> = [Option<T>, ..512];
pub type N1024<T> = [Option<T>, ..1024];


impl<T> StackVecArray<T> for [Option<T>, ..2] {
    impl_stack_vec_array_basic_methods!(2)
    fn new() -> [Option<T>, ..2] {
        [
            None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..4] {
    impl_stack_vec_array_basic_methods!(4)
    fn new() -> [Option<T>, ..4] {
        [
            None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..8] {
    impl_stack_vec_array_basic_methods!(8)
    fn new() -> [Option<T>, ..8] {
        [
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..16] {
    impl_stack_vec_array_basic_methods!(16)
    fn new() -> [Option<T>, ..16] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..32] {
    impl_stack_vec_array_basic_methods!(32)
    fn new() -> [Option<T>, ..32] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..64] {
    impl_stack_vec_array_basic_methods!(64)
    fn new() -> [Option<T>, ..64] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..128] {
    impl_stack_vec_array_basic_methods!(128)
    fn new() -> [Option<T>, ..128] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..256] {
    impl_stack_vec_array_basic_methods!(256)
    fn new() -> [Option<T>, ..256] {
        [
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ]
    }
}

impl<T> StackVecArray<T> for [Option<T>, ..512] {
    impl_stack_vec_array_basic_methods!(512)
    fn new() -> [Option<T>, ..512] {
        [

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,


        ]
    }
}


impl<T> StackVecArray<T> for [Option<T>, ..1024] {
    impl_stack_vec_array_basic_methods!(1024)
    fn new() -> [Option<T>, ..1024] {
        [

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

        ]
    }
}


#[test]
fn it_works() {

    let mut vec: StackVec<N32<uint>> = StackVec::new();
    for i in range(0u, 24) {
        vec.push(i);
    }
    for elem in vec.iter() {
        println!("{}", elem);
    }

}
