use crate::{rvv_type::Vector, Length};

/// Vector Unit-Stride Load Function
#[target_feature(enable = "v")]
pub unsafe fn vlv<V>(base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Load Function
#[target_feature(enable = "v")]
pub unsafe fn vlvtu<V>(dest: V, base: *const V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
#[target_feature(enable = "v")]
pub unsafe fn vsv<V>(base: *mut V::Element, value: V, vl: Length<V>)
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
#[target_feature(enable = "v")]
pub unsafe fn vlsv<V>(base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Vector Strided Load Function
#[target_feature(enable = "v")]
pub unsafe fn vlsvtu<V>(dest: V, base: *const V::Element, bstride: isize, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

pub(crate) mod masked {
    use crate::{rvv_type::Vector, Length};

    /// Vector Unit-Stride Load Function
    #[target_feature(enable = "v")]
    pub unsafe fn vlvm<V>(mask: V::Mask, maskedoff: V, base: *const V::Element, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Vector Unit-Stride Load Function
    #[target_feature(enable = "v")]
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
    #[target_feature(enable = "v")]
    pub unsafe fn vsvm<V>(mask: V::Mask, base: *mut V::Element, value: V, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Vector Strided Load Function
    #[target_feature(enable = "v")]
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
    #[target_feature(enable = "v")]
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
