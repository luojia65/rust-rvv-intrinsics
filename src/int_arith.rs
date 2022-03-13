use crate::{Length, Vector};

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
pub fn vneg<V>(op: V, vl: Length<V>) -> V
where
    V: Vector,
{
    todo!()
}
