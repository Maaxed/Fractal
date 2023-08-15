mod complex;
pub mod function;

pub use complex::*;
pub use function::{Func, Function, Differentiable};

#[cfg(target_arch = "spirv")]
use num_traits::Float;
use num_traits::Pow;

pub fn exp(v: f32) -> f32
{
    v.exp()
}

pub fn ln(v: f32) -> f32
{
    v.ln()
}

pub fn log2(v: f32) -> f32
{
    v.log2()
}

pub fn log(v: f32, base: f32) -> f32
{
    v.log(base)
}

pub fn pow(v: f32, e: f32) -> f32
{
    v.pow(e)
}

pub fn sqrt(v: f32) -> f32
{
    v.sqrt()
}

pub fn floor(v: f32) -> f32
{
    v.floor()
}

pub fn abs(v: f32) -> f32
{
    v.abs()
}
