#![allow(non_camel_case_types)]
use crate::rvv_type::Vector;

/// Set vl and vtype Function
#[inline]
pub fn vsetvl<V>(avl: usize) -> Length<V>
where
    V: Vector,
{
    let sew = core::mem::size_of::<V::Mask>();
    let bytes = unsafe { llvm_vsetvli(avl, sew, 1) }; // todo: grouping
    Length {
        bytes,
        _marker: core::marker::PhantomData,
    }
}

/// Set vl and vtype Function
#[inline]
pub fn vsetvlmax<V>() -> Length<V>
where
    V: Vector,
{
    let sew = core::mem::size_of::<V::Mask>();
    let bytes = unsafe { llvm_vsetvlimax(sew, 1) }; // todo: grouping
    Length {
        bytes,
        _marker: core::marker::PhantomData,
    }
}

/// Vector length configuration
#[derive(Debug)]
pub struct Length<V> {
    bytes: usize,
    _marker: core::marker::PhantomData<V>,
}

impl<V> Clone for Length<V> {
    #[inline]
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

extern "C" {
    #[link_name = "llvm.riscv.vsetvli"]
    fn llvm_vsetvli(avl: usize, sew: usize, lmul: usize) -> usize;
    #[link_name = "llvm.riscv.vsetvlimax"]
    fn llvm_vsetvlimax(sew: usize, lmul: usize) -> usize;
}
