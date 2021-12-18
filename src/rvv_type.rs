#![allow(non_camel_case_types)]

pub trait Vector {
    type Element;
    type Grouping;
    type Mask;
}

pub struct mf8;
pub struct mf4;
pub struct mf2;
pub struct m1;
pub struct m2;
pub struct m4;
pub struct m8;

macro_rules! impl_vector_types {
    ($(($v: ident, $em: ident, $gr: ident, $m: ident),)+) => {
        $(

pub struct $v(());

impl Vector for $v {
    type Element = $em;
    type Grouping = $gr;
    type Mask = $m;
}
        )+
    };
}

pub struct f16(u16);

impl_vector_types! {
    (vint8mf8_t,    i8,     mf8,    vbool64_t),
    (vint8mf4_t,    i8,     mf4,    vbool32_t),
    (vint8mf2_t,    i8,     mf2,    vbool16_t),
    (vint8m1_t,     i8,     m1,     vbool8_t),
    (vint8m2_t,     i8,     m2,     vbool4_t),
    (vint8m4_t,     i8,     m4,     vbool2_t),
    (vint8m8_t,     i8,     m8,     vbool1_t),
    (vint16mf4_t,   i16,    mf4,    vbool64_t),
    (vint16mf2_t,   i16,    mf2,    vbool32_t),
    (vint16m1_t,    i16,    m1,     vbool16_t),
    (vint16m2_t,    i16,    m2,     vbool8_t),
    (vint16m4_t,    i16,    m4,     vbool4_t),
    (vint16m8_t,    i16,    m8,     vbool2_t),
    (vint32mf2_t,   i32,    mf2,    vbool64_t),
    (vint32m1_t,    i32,    m1,     vbool32_t),
    (vint32m2_t,    i32,    m2,     vbool16_t),
    (vint32m4_t,    i32,    m4,     vbool8_t),
    (vint32m8_t,    i32,    m8,     vbool4_t),
    (vint64m1_t,    i64,    m1,     vbool64_t),
    (vint64m2_t,    i64,    m2,     vbool32_t),
    (vint64m4_t,    i64,    m4,     vbool16_t),
    (vint64m8_t,    i64,    m8,     vbool8_t),
    (vuint8mf8_t,   u8,     mf8,    vbool64_t),
    (vuint8mf4_t,   u8,     mf4,    vbool32_t),
    (vuint8mf2_t,   u8,     mf2,    vbool16_t),
    (vuint8m1_t,    u8,     m1,     vbool8_t),
    (vuint8m2_t,    u8,     m2,     vbool4_t),
    (vuint8m4_t,    u8,     m4,     vbool2_t),
    (vuint8m8_t,    u8,     m8,     vbool1_t),
    (vuint16mf4_t,  u16,    mf4,    vbool64_t),
    (vuint16mf2_t,  u16,    mf2,    vbool32_t),
    (vuint16m1_t,   u16,    m1,     vbool16_t),
    (vuint16m2_t,   u16,    m2,     vbool8_t),
    (vuint16m4_t,   u16,    m4,     vbool4_t),
    (vuint16m8_t,   u16,    m8,     vbool2_t),
    (vuint32mf2_t,  u32,    mf2,    vbool64_t),
    (vuint32m1_t,   u32,    m1,     vbool32_t),
    (vuint32m2_t,   u32,    m2,     vbool16_t),
    (vuint32m4_t,   u32,    m4,     vbool8_t),
    (vuint32m8_t,   u32,    m8,     vbool4_t),
    (vuint64m1_t,   u64,    m1,     vbool64_t),
    (vuint64m2_t,   u64,    m2,     vbool32_t),
    (vuint64m4_t,   u64,    m4,     vbool16_t),
    (vuint64m8_t,   u64,    m8,     vbool8_t),
    (vfloat16mf4_t, f16,    mf4,    vbool64_t),
    (vfloat16mf2_t, f16,    mf2,    vbool32_t),
    (vfloat16m1_t,  f16,    m1,     vbool16_t),
    (vfloat16m2_t,  f16,    m2,     vbool8_t),
    (vfloat16m4_t,  f16,    m4,     vbool4_t),
    (vfloat16m8_t,  f16,    m8,     vbool2_t),
    (vfloat32mf2_t, f32,    mf2,    vbool64_t),
    (vfloat32m1_t,  f32,    m1,     vbool32_t),
    (vfloat32m2_t,  f32,    m2,     vbool16_t),
    (vfloat32m4_t,  f32,    m4,     vbool8_t),
    (vfloat32m8_t,  f32,    m8,     vbool4_t),
    (vfloat64m1_t,  f64,    m1,     vbool64_t),
    (vfloat64m2_t,  f64,    m2,     vbool32_t),
    (vfloat64m4_t,  f64,    m4,     vbool16_t),
    (vfloat64m8_t,  f64,    m8,     vbool8_t),
}

pub struct vbool1_t(());
pub struct vbool2_t(());
pub struct vbool4_t(());
pub struct vbool8_t(());
pub struct vbool16_t(());
pub struct vbool32_t(());
pub struct vbool64_t(());
