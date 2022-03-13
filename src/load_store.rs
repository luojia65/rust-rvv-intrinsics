use crate::{Length, rvv_type::Vector};

/// Vector Unit-Stride Load Function
pub unsafe fn vlv<V>(base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub unsafe fn vlvtu<V>(dest: V, base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
pub unsafe fn vsv<V>(base: *mut V::Element, value: V, vl: Length<V>)
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub unsafe fn vlsv<V>(base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
pub unsafe fn vlsvtu<V>(dest: V, base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

pub(crate) mod masked {
    use crate::{Length, rvv_type::Vector};

    /// Vector Unit-Stride Load Function
    pub unsafe fn vlvm<V>(mask: V::Mask, maskedoff: V, base: *const V::Element, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }
    
    /// Vector Unit-Stride Load Function
    pub unsafe fn vlvmt<V>(
        mask: V::Mask,
        maskedoff: V,
        base: *const V::Element,
        vl: Length<V>,
        ta: usize,
    ) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Vector Unit-Stride Store Functions
    pub unsafe fn vsvm<V>(mask: V::Mask, base: *mut V::Element, value: V, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Vector Strided Load Function
    pub unsafe fn vlsvm<V>(
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
    pub unsafe fn vlsvmt<V>(
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

}
