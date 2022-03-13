use crate::{Length, rvv_type::Vector};

/// Adds two vectors
pub fn vaddvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Adds a vector with a value on each element
pub fn vaddvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Subtracts one vector by another vector
pub fn vsubvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Subtracts one vector by a value on each element
pub fn vsubvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Subtracts elements on one vector from a value
pub fn vrsubvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

/// Negate all elements on vector
pub fn vnegv<V>(op: V, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}

pub(crate) mod masked {
    use crate::{Length, rvv_type::Vector};
    /// Adds two vectors with mask
    pub fn vaddvvm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Adds a vector with a value on masked elements
    pub fn vaddvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Subtracts one vector by another vector with mask
    pub fn vsubvvm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Subtracts one vector by a value on masked element
    pub fn vsubvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Subtracts masked elements on one vector from a value
    pub fn vrsubvxm<V>(mask: V::Mask, maskedoff: V, op1: V, op2: V::Element, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }

    /// Negate masked elements on vector
    pub fn vnegvm<V>(mask: V::Mask, maskedoff: V, op: V, vl: Length<V>) -> V
    where
        V: Vector,
    {
        todo!()
    }
}
