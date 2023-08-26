use crate::math::*;
use super::{escape_time_method::*, FractalParams};

const ITERATION_COUNT: u32 = 1024;

pub fn multibrot3<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    compute_escape_time(pos, params, ITERATION_COUNT, 1.0e6, Some(3.0), |z, c|
    {
        (z.squared() * z) + c
    })
}
