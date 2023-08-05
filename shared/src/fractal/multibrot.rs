use crate::complex::Complex;
use super::escape_time_method::{compute_escape_time, Params};

const ITERATION_COUNT: u32 = 1024;

pub fn multibrot3(pos: Complex, params: Params) -> f32
{
    compute_escape_time(pos, params, ITERATION_COUNT, 2.0, |z, c|
    {
        (z.squared() * z) + c
    })
}
