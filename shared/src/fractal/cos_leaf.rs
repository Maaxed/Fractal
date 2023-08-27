use crate::math::*;
use super::{escape_time_method::*, FractalParams};

const ITERATION_COUNT: u32 = 1024;

pub fn cos_leaf<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    compute_escape_time_fractal(pos, params, ITERATION_COUNT, 100.0, None, |z, c|
    {
        (z / c).cos()
    })
}
