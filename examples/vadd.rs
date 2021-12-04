use rust_rvv_intrinsics::*;

fn add<const N: usize>(a: &[u8; N], b: &[u8; N], c: &mut [u8; N]) {
    vsetvl::<e8, m8>(4);
    let vs1 = vlv::<_, m8>(a.as_ptr(), 4);
    let vs2 = vlv::<_, m8>(b.as_ptr(), 4);
    let vd = vaddvv(vs1, vs2, 4);
    vsv::<_, m8>(c.as_mut_ptr(), vd, 4);
}

fn main() {
    let src1 = [1, 2, 3, 4];
    let src2 = [5, 6, 7, 8];
    let mut dst = [0; 4];
    add(&src1, &src2, &mut dst);
}
