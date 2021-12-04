use crate::{ValidElemType, V};

/// Vector Unit-Stride Load Function
pub fn vlv<T, M>(base: *const T, vl: usize) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvtu<T, M>(dest: V<T, M>, base: *const T, vl: usize) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvm<T, M>(
    mask: <(T, M) as ValidElemType>::Mask,
    maskedoff: V<T, M>,
    base: *const T,
    vl: usize,
) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Unit-Stride Load Function
pub fn vlvmt<T, M>(
    mask: <(T, M) as ValidElemType>::Mask,
    maskedoff: V<T, M>,
    base: *const T,
    vl: usize,
    ta: usize,
) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
pub fn vsv<T, M>(base: *mut T, value: V<T, M>, vl: usize)
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Unit-Stride Store Functions
pub fn vsvm<T, M>(
    mask: <(T, M) as ValidElemType>::Mask,
    base: *mut T,
    value: V<T, M>,
    vl: usize,
) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsv<T, M>(base: *const T, bstride: isize, vl: usize) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvtu<T, M>(dest: V<T, M>, base: *const T, bstride: isize, vl: usize) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvm<T, M>(
    mask: <(T, M) as ValidElemType>::Mask,
    maskedoff: V<T, M>,
    base: *const T,
    bstride: isize,
    vl: usize,
) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}

/// Vector Strided Load Function
pub fn vlsvmt<T, M>(
    mask: <(T, M) as ValidElemType>::Mask,
    maskedoff: V<T, M>,
    base: *const T,
    bstride: isize,
    vl: usize,
    ta: usize,
) -> V<T, M>
where
    (T, M): ValidElemType,
{
    todo!()
}
