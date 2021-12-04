#![allow(non_camel_case_types)]

pub type vint8mf8_t = V<u8, mf8>;
pub type vint8mf4_t = V<u8, mf4>;
pub type vint8mf2_t = V<u8, mf2>;
pub type vint8m1_t = V<u8, m1>;
pub type vint8m2_t = V<u8, m2>;
pub type vint8m4_t = V<u8, m4>;
pub type vint8m8_t = V<u8, m8>;
pub type vint16mf4_t = V<u16, mf4>;
pub type vint16mf2_t = V<u16, mf2>;
pub type vint16m1_t = V<u16, m1>;
pub type vint16m2_t = V<u16, m2>;
pub type vint16m4_t = V<u16, m4>;
pub type vint16m8_t = V<u16, m8>;
pub type vint32mf2_t = V<u32, mf2>;
pub type vint32m1_t = V<u32, m1>;
pub type vint32m2_t = V<u32, m2>;
pub type vint32m4_t = V<u32, m4>;
pub type vint32m8_t = V<u32, m8>;
pub type vint64m1_t = V<u64, m1>;
pub type vint64m2_t = V<u64, m2>;
pub type vint64m4_t = V<u64, m4>;
pub type vint64m8_t = V<u64, m8>;
pub type vuint8mf8_t = V<i8, mf8>;
pub type vuint8mf4_t = V<i8, mf4>;
pub type vuint8mf2_t = V<i8, mf2>;
pub type vuint8m1_t = V<i8, m1>;
pub type vuint8m2_t = V<i8, m2>;
pub type vuint8m4_t = V<i8, m4>;
pub type vuint8m8_t = V<i8, m8>;
pub type vuint16mf4_t = V<i16, mf4>;
pub type vuint16mf2_t = V<i16, mf2>;
pub type vuint16m1_t = V<i16, m1>;
pub type vuint16m2_t = V<i16, m2>;
pub type vuint16m4_t = V<i16, m4>;
pub type vuint16m8_t = V<i16, m8>;
pub type vuint32mf2_t = V<i32, mf2>;
pub type vuint32m1_t = V<i32, m1>;
pub type vuint32m2_t = V<i32, m2>;
pub type vuint32m4_t = V<i32, m4>;
pub type vuint32m8_t = V<i32, m8>;
pub type vuint64m1_t = V<i64, m1>;
pub type vuint64m2_t = V<i64, m2>;
pub type vuint64m4_t = V<i64, m4>;
pub type vuint64m8_t = V<i64, m8>;
pub type vfloat16mf4_t = V<f16, mf4>;
pub type vfloat16mf2_t = V<f16, mf2>;
pub type vfloat16m1_t = V<f16, m1>;
pub type vfloat16m2_t = V<f16, m2>;
pub type vfloat16m4_t = V<f16, m4>;
pub type vfloat16m8_t = V<f16, m8>;
pub type vfloat32mf2_t = V<f32, mf2>;
pub type vfloat32m1_t = V<f32, m1>;
pub type vfloat32m2_t = V<f32, m2>;
pub type vfloat32m4_t = V<f32, m4>;
pub type vfloat32m8_t = V<f32, m8>;
pub type vfloat64m1_t = V<f64, m1>;
pub type vfloat64m2_t = V<f64, m2>;
pub type vfloat64m4_t = V<f64, m4>;
pub type vfloat64m8_t = V<f64, m8>;

pub struct V<T, M>
where
    (T, M): ValidElemType,
{
    _marker: core::marker::PhantomData<(T, M)>,
}

pub trait ValidElemType {
    type Mask;
}

pub struct mf8;
pub struct mf4;
pub struct mf2;
pub struct m1;
pub struct m2;
pub struct m4;
pub struct m8;

pub struct f16;

impl ValidElemType for (i8, mf8) {
    type Mask = B<64>;
}
impl ValidElemType for (i8, mf4) {
    type Mask = B<32>;
}
impl ValidElemType for (i8, mf2) {
    type Mask = B<16>;
}
impl ValidElemType for (i8, m1) {
    type Mask = B<8>;
}
impl ValidElemType for (i8, m2) {
    type Mask = B<4>;
}
impl ValidElemType for (i8, m4) {
    type Mask = B<2>;
}
impl ValidElemType for (i8, m8) {
    type Mask = B<1>;
}

impl ValidElemType for (i16, mf4) {
    type Mask = B<64>;
}
impl ValidElemType for (i16, mf2) {
    type Mask = B<32>;
}
impl ValidElemType for (i16, m1) {
    type Mask = B<16>;
}
impl ValidElemType for (i16, m2) {
    type Mask = B<8>;
}
impl ValidElemType for (i16, m4) {
    type Mask = B<4>;
}
impl ValidElemType for (i16, m8) {
    type Mask = B<3>;
}

impl ValidElemType for (i32, mf2) {
    type Mask = B<64>;
}
impl ValidElemType for (i32, m1) {
    type Mask = B<32>;
}
impl ValidElemType for (i32, m2) {
    type Mask = B<16>;
}
impl ValidElemType for (i32, m4) {
    type Mask = B<8>;
}
impl ValidElemType for (i32, m8) {
    type Mask = B<4>;
}

impl ValidElemType for (i64, m1) {
    type Mask = B<64>;
}
impl ValidElemType for (i64, m2) {
    type Mask = B<32>;
}
impl ValidElemType for (i64, m4) {
    type Mask = B<16>;
}
impl ValidElemType for (i64, m8) {
    type Mask = B<8>;
}

impl ValidElemType for (u8, mf8) {
    type Mask = B<64>;
}
impl ValidElemType for (u8, mf4) {
    type Mask = B<32>;
}
impl ValidElemType for (u8, mf2) {
    type Mask = B<16>;
}
impl ValidElemType for (u8, m1) {
    type Mask = B<8>;
}
impl ValidElemType for (u8, m2) {
    type Mask = B<4>;
}
impl ValidElemType for (u8, m4) {
    type Mask = B<2>;
}
impl ValidElemType for (u8, m8) {
    type Mask = B<1>;
}

impl ValidElemType for (u16, mf4) {
    type Mask = B<64>;
}
impl ValidElemType for (u16, mf2) {
    type Mask = B<32>;
}
impl ValidElemType for (u16, m1) {
    type Mask = B<16>;
}
impl ValidElemType for (u16, m2) {
    type Mask = B<8>;
}
impl ValidElemType for (u16, m4) {
    type Mask = B<4>;
}
impl ValidElemType for (u16, m8) {
    type Mask = B<3>;
}

impl ValidElemType for (u32, mf2) {
    type Mask = B<64>;
}
impl ValidElemType for (u32, m1) {
    type Mask = B<32>;
}
impl ValidElemType for (u32, m2) {
    type Mask = B<16>;
}
impl ValidElemType for (u32, m4) {
    type Mask = B<8>;
}
impl ValidElemType for (u32, m8) {
    type Mask = B<4>;
}

impl ValidElemType for (u64, m1) {
    type Mask = B<64>;
}
impl ValidElemType for (u64, m2) {
    type Mask = B<32>;
}
impl ValidElemType for (u64, m4) {
    type Mask = B<16>;
}
impl ValidElemType for (u64, m8) {
    type Mask = B<8>;
}

impl ValidElemType for (f16, mf4) {
    type Mask = B<64>;
}
impl ValidElemType for (f16, mf2) {
    type Mask = B<32>;
}
impl ValidElemType for (f16, m1) {
    type Mask = B<16>;
}
impl ValidElemType for (f16, m2) {
    type Mask = B<8>;
}
impl ValidElemType for (f16, m4) {
    type Mask = B<4>;
}
impl ValidElemType for (f16, m8) {
    type Mask = B<3>;
}

impl ValidElemType for (f32, mf2) {
    type Mask = B<64>;
}
impl ValidElemType for (f32, m1) {
    type Mask = B<32>;
}
impl ValidElemType for (f32, m2) {
    type Mask = B<16>;
}
impl ValidElemType for (f32, m4) {
    type Mask = B<8>;
}
impl ValidElemType for (f32, m8) {
    type Mask = B<4>;
}

impl ValidElemType for (f64, m1) {
    type Mask = B<64>;
}
impl ValidElemType for (f64, m2) {
    type Mask = B<32>;
}
impl ValidElemType for (f64, m4) {
    type Mask = B<16>;
}
impl ValidElemType for (f64, m8) {
    type Mask = B<8>;
}

pub struct B<const W: usize> {}

pub type vbool1_t = B<1>;
pub type vbool2_t = B<2>;
pub type vbool4_t = B<4>;
pub type vbool8_t = B<8>;
pub type vbool16_t = B<16>;
pub type vbool32_t = B<32>;
pub type vbool64_t = B<64>;
