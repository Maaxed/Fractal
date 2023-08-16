mod complex;
pub mod function;

pub use complex::*;
pub use function::{Func, Function, Differentiable};

use num_traits::{Float, Pow};

pub trait Exp
{
    fn squared(self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn pow(self, exp: Self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
}

pub trait Trigo
{
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

impl<T: Float> Trigo for T
{
    fn sin(self) -> Self
    {
        Float::sin(self)
    }

    fn cos(self) -> Self
    {
        Float::cos(self)
    }
}


impl<T: Float + Pow<T, Output = T>> Exp for T
{
    fn squared(self) -> Self
    {
        self * self    
    }

    fn sqrt(self) -> Self
    {
        Float::sqrt(self)
    }

    fn exp(self) -> Self
    {
        Float::exp(self)
    }

    fn pow(self, exp: Self) -> Self
    {
        Pow::pow(self, exp)
    }

    fn ln(self) -> Self
    {
        Float::ln(self)
    }

    fn log(self, base: Self) -> Self
    {
        Float::log(self, base)
    }
}

pub fn exp(v: f32) -> f32
{
    Exp::exp(v)
}

pub fn ln(v: f32) -> f32
{
    Exp::ln(v)
}

pub fn log2(v: f32) -> f32
{
    v.log2()
}

pub fn log(v: f32, base: f32) -> f32
{
    Exp::log(v, base)
}

pub fn pow(v: f32, e: f32) -> f32
{
    Exp::pow(v, e)
}

pub fn sqrt(v: f32) -> f32
{
    Exp::sqrt(v)
}

pub fn floor(v: f32) -> f32
{
    v.floor()
}

pub fn abs(v: f32) -> f32
{
    v.abs()
}
