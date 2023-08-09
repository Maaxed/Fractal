use crate::complex::Complex;
use super::escape_time_method::*;

const ITERATION_COUNT: u32 = 1024;

pub fn multibrot3(pos: Complex, params: Params) -> EscapeResult
{
    compute_escape_time(pos, params, ITERATION_COUNT, 1.0e6, Some(3.0), |z, c|
    {
        (z.squared() * z) + c
    })
}
