use crate::{
    rvv_type::{IntVector, Widenable},
    Length,
};

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

/// Widening adds two vectors
pub fn vwaddvv<V>(op1: V, op2: V, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Widening adds one vector and one value from variable
pub fn vwaddvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Adds a vector to a widened vector
pub fn vwaddwv<V>(op1: V::Wide, op2: V, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Adds one value from variable to a widened vector
pub fn vwaddwx<V>(op1: V::Wide, op2: V::Element, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Widening subtracts one vector by another
pub fn vwsubvv<V>(op1: V, op2: V, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Widening substracts one vector by one value from variable
pub fn vwsubvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Substracts a widened vector by a vector
pub fn vwsubwv<V>(op1: V::Wide, op2: V, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Substracts a widened vector by a value form variable
pub fn vwsubwx<V>(op1: V::Wide, op2: V::Element, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Widen a vector
pub fn vwcvt<V>(op: V, vl: Length<V>) -> V::Wide
where
    V: Widenable,
{
    todo!()
}

/// Ands two vectors
pub fn vandvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Ands a vector with value from variable on each element
pub fn vandvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Ands a vector with a constant value on each element
pub fn vandvi<V, const I: u8>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Ors two vectors
pub fn vorvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Ors a vector with value from variable on each element
pub fn vorvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Ors a vector with a constant value on each element
pub fn vorvi<V, const I: u8>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Xors two vectors
pub fn vxorvv<V>(op1: V, op2: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Xors a vector with value from variable on each element
pub fn vxorvx<V>(op1: V, op2: V::Element, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Xors a vector with a constant value on each element
pub fn vxorvi<V, const I: u8>(op: V, vl: Length<V>) -> V
where
    V: IntVector,
{
    todo!()
}

/// Flips all bits in vector
pub fn vnotv<V>(op: V, vl: Length<V>) -> V
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
