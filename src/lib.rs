#![feature(let_chains)]
#![feature(isqrt)]
#![feature(associated_type_defaults)]
#![feature(unboxed_closures)]

extern crate core;

pub mod common;
pub mod top;
pub mod algebra;
pub mod aprogression;
pub mod gprogression;
pub mod vector_algebra;
pub mod func_analysis;
pub mod utils;
pub mod matrices;
pub mod im_numbers;

#[doc(hidden)]
#[allow(clippy::module_inception)]
pub(crate) mod tests;
