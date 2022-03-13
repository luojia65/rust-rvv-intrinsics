use rust_rvv_intrinsics::*;

fn add<const N: usize>(a: &[u8; N], b: &[u8; N], c: &mut [u8; N]) {
    let mut n = N;
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let mut c_ptr = c.as_mut_ptr();
    while n > 0 {
        let vl = vsetvl::<vuint8mf8_t>(n);
        n -= vl.as_bytes();
        unsafe {
            let vs1 = vlv(a_ptr, vl);
            let vs2 = vlv(b_ptr, vl);
            let vd = vaddvv(vs1, vs2, vl);
            vsv(c_ptr, vd, vl);
            a_ptr = a_ptr.add(vl.as_bytes());
            b_ptr = b_ptr.add(vl.as_bytes());
            c_ptr = c_ptr.add(vl.as_bytes());
        }
    }
}

pub fn main() {
    let src1 = [1, 2, 3, 4];
    let src2 = [5, 6, 7, 8];
    let mut dst = [0; 4];
    add(&src1, &src2, &mut dst);
}

extern "C" {}
