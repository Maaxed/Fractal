use crate::math::*;
use super::{escape_time_method::*, FractalParams};
use num_traits::Zero;

const ITERATION_COUNT: u32 = 1024;


#[derive(Clone, Copy)]
struct Conjugate;
#[derive(Clone, Copy)]
struct DConjugate;

impl<C: ComplexNumber> Function<C> for Conjugate
{
    type Output = C;

    fn get(&self, z: C) -> Self::Output
    {
        z.conjugate()
    }
}

impl<C: ComplexNumber> Differentiable<C> for Conjugate
{
    type Derivative = DConjugate;

    fn derivative(&self) -> Func<Self::Derivative>
    {
        Func(DConjugate)
    }
}

impl<C: ComplexNumber> Function<C> for DConjugate
{
    type Output = C;

    fn get(&self, z: C) -> Self::Output
    {
        let ms = z.modulus_squared();
        if ms == Zero::zero()
        {
            C::ONE
        }
        else
        {
            z.conjugate().squared() / ms
        }
    }
}


pub fn tricorn<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    compute_escape_time_fractal(pos, params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        Func(Conjugate).compose(z).squared() + c
    })
}
