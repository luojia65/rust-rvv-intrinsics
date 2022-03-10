#![allow(non_camel_case_types)]
use crate::Vector;

/// Set vl and vtype Function
pub fn vsetvl<V>(avl: usize) -> Length<V>
where
    V: Vector,
{
    let bytes = arch_vsetvli::<0, 0>(avl);
    Length { bytes, _marker: core::marker::PhantomData }
}

#[inline]
fn arch_vsetvli<const SEW: usize, const LMUL: usize>(avl: usize) -> usize {
    unsafe { llvm_vsetvli(avl, SEW, LMUL) }
}

/// Set vl and vtype Function
pub fn vsetvlmax<V>() -> Length<V>
where
    V: Vector,
{
    todo!()
}

/// Vector length configuration
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

extern "C" {
    #[link_name = "llvm.riscv.vsetvli"]
    fn llvm_vsetvli(avl: usize, sew: usize, lmul: usize) -> usize;
}
