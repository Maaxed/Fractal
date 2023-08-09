use crate::math::Complex;
use super::escape_time_method::*;

const ITERATION_COUNT: u32 = 1024;

pub fn cos_leaf(pos: Complex, params: Params) -> EscapeResult
{
    compute_escape_time(pos, params, ITERATION_COUNT, 100.0, None, |z, c|
    {
        (z / c).cos()
    })
}
