use rust_rvv_intrinsics::*;
use core::mem::size_of;

fn add<const N: usize>(a: &[u8; N], b: &[u8; N], c: &mut [u8; N]) {
    let mut n = N;
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let mut c_ptr = c.as_mut_ptr();
    while n > 0 {
        let vl = vsetvl::<e8, m8>(n);
        n -= vl;
        let vs1 = vlv::<u8, m8>(a_ptr, vl);
        a_ptr = unsafe { a_ptr.add(size_of::<u8>() * vl) };
        let vs2 = vlv::<u8, m8>(b_ptr, vl);
        b_ptr = unsafe { b_ptr.add(size_of::<u8>() * vl) };
        let vd = vaddvv(vs1, vs2, vl);
        vsv::<_, m8>(c_ptr, vd, vl);
        c_ptr = unsafe { c_ptr.add(size_of::<u8>() * vl) };
    }
}

fn main() {
    let src1 = [1, 2, 3, 4];
    let src2 = [5, 6, 7, 8];
    let mut dst = [0; 4];
    add(&src1, &src2, &mut dst);
}
