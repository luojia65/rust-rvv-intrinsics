use crate::{rvv_type::IntVector, Length};

/// Adds two vectors
pub fn vaddvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Adds a vector with value from variable on each element
pub fn vaddvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Adds a vector with a constant value on each element
pub fn vaddvi<V, const I: u8>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Subtracts one vectors by another vectors
pub fn vsubvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Subtracts one vectors by a value on each element
pub fn vsubvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Subtracts elements on one vectors from a value from variable
pub fn vrsubvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Subtracts elements on one vectors from a constant value
pub fn vrsubvi<V, const I: u8>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Negate all elements on vectors
pub fn vnegv<V>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

pub(crate) mod masked {
    use crate::{rvv_type::IntVector, Length};
    /// Adds two vectors with mask
    pub fn vaddvvm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }

    /// Adds a vectors with a value on masked elements
    pub fn vaddvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }

    /// Subtracts one vectors by another vectors with mask
    pub fn vsubvvm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }

    /// Subtracts one vectors by a value on masked element
    pub fn vsubvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }

    /// Subtracts masked elements on one vectors from a value
    pub fn vrsubvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }

    /// Negate masked elements on vectors
    pub fn vnegvm<V>(mask: V::Mask, maskedoff: V, op: V, vl: Length<V>) -> V
    where
        V: IntVector,
    {
        todo!()
    }
}
