use crate::math::*;
use super::{escape_time_method::*, FractalParams};

const ITERATION_COUNT: u32 = 1024;

pub fn tricorn<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    compute_escape_time_fractal(pos, params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        z.conjugate().squared() + c
    })
}
