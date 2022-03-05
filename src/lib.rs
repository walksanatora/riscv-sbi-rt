//! A mininal runtime / startup for OpenSBI on RISC-V.

#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod io;
mod log;
mod runtime;
pub mod sbi;

pub use opensbi_rt_macros::entry;
