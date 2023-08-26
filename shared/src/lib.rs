#![no_std]
#![deny(warnings)]

#![cfg_attr(target_arch = "spirv", feature(repr_simd))]

pub mod math;
pub mod fractal;
pub mod render;
pub mod compute;
