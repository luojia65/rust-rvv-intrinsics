use crate::{Length, Vector};

/// Vector Unit-Stride Load Function
pub fn vlv<V>(base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvtu<V>(dest: V, base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvm<V>(mask: V::Mask, maskedoff: V, base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvmt<V>(mask: V::Mask, maskedoff: V, base: *const V::Element, vl: Length<V>, ta: usize) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
pub fn vsv<V>(base: *mut V::Element, value: V, vl: Length<V>)
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
pub fn vsvm<V>(mask: V::Mask, base: *mut V::Element, value: V, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsv<V>(base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvtu<V>(dest: V, base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvm<V>(
    mask: V::Mask,
    maskedoff: V,
    base: *const V::Element,
    bstride: isize,
    vl: Length<V>,
) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvmt<V>(
    mask: V::Mask,
    maskedoff: V,
    base: *const V::Element,
    bstride: isize,
    vl: Length<V>,
    ta: usize,
) -> V
where
    V: Vector,
{
    todo!()
}
