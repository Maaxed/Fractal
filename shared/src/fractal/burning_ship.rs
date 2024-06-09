use crate::math::*;
use super::{escape_time_method::*, FractalParams};
use num_traits::Zero;

pub const ITERATION_COUNT: u32 = 1024;


#[derive(Clone, Copy)]
struct Abs;
#[derive(Clone, Copy)]
struct DAbs;

impl<C: ComplexNumber> Function<C> for Abs
{
    type Output = C;

    fn get(&self, z: C) -> Self::Output
    {
        C::from_vector(z.to_vector().abs())
    }
}

impl<C: ComplexNumber> Differentiable<C> for Abs
{
    type Derivative = DAbs;

    fn derivative(&self) -> Func<Self::Derivative>
    {
        Func(DAbs)
    }
}

impl<C: ComplexNumber> Function<C> for DAbs
{
    type Output = C;

    fn get(&self, z: C) -> Self::Output
    {
        match (z.re() >= Zero::zero(), z.im() >= Zero::zero())
        {
            (true, true) => C::ONE,
            (false, false) => -C::ONE,
            (true, false) => z.conjugate() / z,
            (false, true) => -z.conjugate() / z,
        }
    }
}


pub fn burning_ship<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    compute_escape_time_fractal(pos, params, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        Func(Abs).compose(z).squared() + c
    })
}
