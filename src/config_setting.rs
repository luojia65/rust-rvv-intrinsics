#![allow(non_camel_case_types)]
use crate::Vector;

/// Set vl and vtype Function
pub fn vsetvl<V>(avl: usize) -> Length<V>
where
    V: Vector,
{
    todo!()
}

/// Set vl and vtype Function
pub fn vsetvlmax<V>() -> Length<V>
where
    V: Vector,
{
    todo!()
}

#[derive(Debug)]
pub struct Length<V> {
    bytes: usize,
    _marker: core::marker::PhantomData<V>,
}

impl<V> Clone for Length<V> {
    fn clone(&self) -> Self {
        Length {
            bytes: self.bytes,
            _marker: core::marker::PhantomData,
        }
    }
}
impl<V> Copy for Length<V> {}

impl<V> Length<V> {
    #[inline]
    pub fn as_bytes(&self) -> usize {
        self.bytes
    }
}
