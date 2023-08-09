use crate::math::Complex;
use super::escape_time_method::*;

const ITERATION_COUNT: u32 = 1024;

pub fn tricorn(pos: Complex, params: Params) -> EscapeResult
{
    compute_escape_time(pos, params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        z.conjugate().squared() + c
    })
}
