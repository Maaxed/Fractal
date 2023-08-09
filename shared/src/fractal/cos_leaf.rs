use crate::complex::Complex;
use super::escape_time_method::{compute_escape_time, Params};

const ITERATION_COUNT: u32 = 1024;

pub fn cos_leaf(pos: Complex, params: Params) -> f32
{
    compute_escape_time(pos, params, ITERATION_COUNT, 100.0, None, |z, c|
    {
        (z / c).cos()
    })
}
