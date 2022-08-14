#![allow(non_camel_case_types)]

pub trait Vector {
    type Element;
    type Grouping;
    type Mask;
}

pub trait IntVector: Vector {}

pub trait Widenable: Vector {
    type Wide;
}

pub struct mf8;
pub struct mf4;
pub struct mf2;
pub struct m1;
pub struct m2;
pub struct m4;
pub struct m8;

macro_rules! impl_vector_types {
    ($($t: tt,)+) => {
        $(
            impl_vector_types!($t);
        )+
    };
    ((@default $v: ident, $em: ident, $gr: ident, $m: ident)) => {
        pub struct $v(());
        impl Vector for $v {
            type Element = $em;
            type Grouping = $gr;
            type Mask = $m;
        }
    };
    ((@integer $v: ident, $wi: tt, $na: tt, $em: ident, $gr: ident, $m: ident)) => {
        impl_vector_types!((@default $v, $em, $gr, $m));
        impl IntVector for $v {}
        impl_vector_types!(@widen $v, $wi);
    };
    (@widen $v:ident, x) => {};
    (@widen $v:ident, $wi: ident) => {
        impl Widenable for $v {
            type Wide = $wi;
        }
    };
}

pub struct f16(u16);

impl_vector_types! {
    (@integer   vint8mf8_t,    vint16mf4_t, x,  i8,     mf8,    vbool64_t),
    (@integer   vint8mf4_t,    vint16mf2_t, x,  i8,     mf4,    vbool32_t),
    (@integer   vint8mf2_t,    vint16m1_t,  x,  i8,     mf2,    vbool16_t),
    (@integer   vint8m1_t,     vint16m2_t,  x,  i8,     m1,     vbool8_t),
    (@integer   vint8m2_t,     vint16m4_t,  x,  i8,     m2,     vbool4_t),
    (@integer   vint8m4_t,     vint16m8_t,  x,  i8,     m4,     vbool2_t),
    (@integer   vint8m8_t,     x,           x,  i8,     m8,     vbool1_t),
    (@integer   vint16mf4_t,   vint32mf2_t, x,  i16,    mf4,    vbool64_t),
    (@integer   vint16mf2_t,   vint32m1_t,  x,  i16,    mf2,    vbool32_t),
    (@integer   vint16m1_t,    vint32m2_t,  x,  i16,    m1,     vbool16_t),
    (@integer   vint16m2_t,    vint32m4_t,  x,  i16,    m2,     vbool8_t),
    (@integer   vint16m4_t,    vint32m8_t,  x,  i16,    m4,     vbool4_t),
    (@integer   vint16m8_t,    x,           x,  i16,    m8,     vbool2_t),
    (@integer   vint32mf2_t,   vint64m1_t,  x,  i32,    mf2,    vbool64_t),
    (@integer   vint32m1_t,    vint64m2_t,  x,  i32,    m1,     vbool32_t),
    (@integer   vint32m2_t,    vint64m4_t,  x,  i32,    m2,     vbool16_t),
    (@integer   vint32m4_t,    vint64m8_t,  x,  i32,    m4,     vbool8_t),
    (@integer   vint32m8_t,    x,           x,  i32,    m8,     vbool4_t),
    (@integer   vint64m1_t,    x,           x,  i64,    m1,     vbool64_t),
    (@integer   vint64m2_t,    x,           x,  i64,    m2,     vbool32_t),
    (@integer   vint64m4_t,    x,           x,  i64,    m4,     vbool16_t),
    (@integer   vint64m8_t,    x,           x,  i64,    m8,     vbool8_t),
    (@integer   vuint8mf8_t,   vuint16mf4_t,x,  u8,     mf8,    vbool64_t),
    (@integer   vuint8mf4_t,   vuint16mf2_t,x,  u8,     mf4,    vbool32_t),
    (@integer   vuint8mf2_t,   vuint16m1_t, x,  u8,     mf2,    vbool16_t),
    (@integer   vuint8m1_t,    vuint16m2_t, x,  u8,     m1,     vbool8_t),
    (@integer   vuint8m2_t,    vuint16m4_t, x,  u8,     m2,     vbool4_t),
    (@integer   vuint8m4_t,    vuint16m8_t, x,  u8,     m4,     vbool2_t),
    (@integer   vuint8m8_t,    x,           x,  u8,     m8,     vbool1_t),
    (@integer   vuint16mf4_t,  vuint32mf2_t,x,  u16,    mf4,    vbool64_t),
    (@integer   vuint16mf2_t,  vuint32m1_t, x,  u16,    mf2,    vbool32_t),
    (@integer   vuint16m1_t,   vuint32m2_t, x,  u16,    m1,     vbool16_t),
    (@integer   vuint16m2_t,   vuint32m4_t, x,  u16,    m2,     vbool8_t),
    (@integer   vuint16m4_t,   vuint32m8_t, x,  u16,    m4,     vbool4_t),
    (@integer   vuint16m8_t,   x,           x,  u16,    m8,     vbool2_t),
    (@integer   vuint32mf2_t,  vuint64m1_t, x,  u32,    mf2,    vbool64_t),
    (@integer   vuint32m1_t,   vuint64m2_t, x,  u32,    m1,     vbool32_t),
    (@integer   vuint32m2_t,   vuint64m4_t, x,  u32,    m2,     vbool16_t),
    (@integer   vuint32m4_t,   vuint64m8_t, x,  u32,    m4,     vbool8_t),
    (@integer   vuint32m8_t,   x,           x,  u32,    m8,     vbool4_t),
    (@integer   vuint64m1_t,   x,           x,  u64,    m1,     vbool64_t),
    (@integer   vuint64m2_t,   x,           x,  u64,    m2,     vbool32_t),
    (@integer   vuint64m4_t,   x,           x,  u64,    m4,     vbool16_t),
    (@integer   vuint64m8_t,   x,           x,  u64,    m8,     vbool8_t),

    (@default   vfloat16mf4_t, f16,    mf4,    vbool64_t),
    (@default   vfloat16mf2_t, f16,    mf2,    vbool32_t),
    (@default   vfloat16m1_t,  f16,    m1,     vbool16_t),
    (@default   vfloat16m2_t,  f16,    m2,     vbool8_t),
    (@default   vfloat16m4_t,  f16,    m4,     vbool4_t),
    (@default   vfloat16m8_t,  f16,    m8,     vbool2_t),
    (@default   vfloat32mf2_t, f32,    mf2,    vbool64_t),
    (@default   vfloat32m1_t,  f32,    m1,     vbool32_t),
    (@default   vfloat32m2_t,  f32,    m2,     vbool16_t),
    (@default   vfloat32m4_t,  f32,    m4,     vbool8_t),
    (@default   vfloat32m8_t,  f32,    m8,     vbool4_t),
    (@default   vfloat64m1_t,  f64,    m1,     vbool64_t),
    (@default   vfloat64m2_t,  f64,    m2,     vbool32_t),
    (@default   vfloat64m4_t,  f64,    m4,     vbool16_t),
    (@default   vfloat64m8_t,  f64,    m8,     vbool8_t),
}

pub struct vbool1_t(());
pub struct vbool2_t(());
pub struct vbool4_t(());
pub struct vbool8_t(());
pub struct vbool16_t(());
pub struct vbool32_t(());
pub struct vbool64_t(());
