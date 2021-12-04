#![allow(non_camel_case_types)]
use crate::{m1, m2, m4, m8, mf2, mf4, mf8};

pub struct e8;
pub struct e16;
pub struct e32;
pub struct e64;

/// Set vl and vtype Function
pub fn vsetvl<E, M>(avl: usize)
where
    (E, M): ValidConfig,
{
    todo!()
}

/// Set vl and vtype Function
pub fn vsetvlmax<E, M>()
where
    (E, M): ValidConfig,
{
    todo!()
}

pub trait ValidConfig {}

impl ValidConfig for (e8, mf8) {}
impl ValidConfig for (e8, mf4) {}
impl ValidConfig for (e8, mf2) {}
impl ValidConfig for (e8, m1) {}
impl ValidConfig for (e8, m2) {}
impl ValidConfig for (e8, m4) {}
impl ValidConfig for (e8, m8) {}
impl ValidConfig for (e16, mf4) {}
impl ValidConfig for (e16, mf2) {}
impl ValidConfig for (e16, m1) {}
impl ValidConfig for (e16, m2) {}
impl ValidConfig for (e16, m4) {}
impl ValidConfig for (e16, m8) {}
impl ValidConfig for (e32, mf2) {}
impl ValidConfig for (e32, m1) {}
impl ValidConfig for (e32, m2) {}
impl ValidConfig for (e32, m4) {}
impl ValidConfig for (e32, m8) {}
impl ValidConfig for (e64, m1) {}
impl ValidConfig for (e64, m2) {}
impl ValidConfig for (e64, m4) {}
impl ValidConfig for (e64, m8) {}
