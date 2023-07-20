use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn tricorn(pos: Complex) -> f32
{
    compute_escape_time(ITERATION_COUNT, 2.0, Complex::ZERO, pos, |z, c|
    {
        z.conjugate().squared() + c
    })
}
