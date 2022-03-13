#![allow(unused_variables)]
#![feature(link_llvm_intrinsics)]
#![no_std]

mod rvv_type;
mod config_setting;
pub use config_setting::*;
mod load_store;
pub use load_store::*;

mod int_arith;
pub use int_arith::*;

/// RISC-V vector types
pub mod types {
    pub use crate::rvv_type::*;
}

/// Masked operations
pub mod mask {
    pub use crate::load_store::masked::*;
    pub use crate::int_arith::masked::*;
}
