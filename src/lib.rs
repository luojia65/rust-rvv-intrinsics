#![allow(unused_variables)]
#![feature(link_llvm_intrinsics)]
#![feature(riscv_target_feature)]
#![no_std]

mod config_setting;
mod rvv_type;
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
    pub use crate::int_arith::masked::*;
    pub use crate::load_store::masked::*;
}
