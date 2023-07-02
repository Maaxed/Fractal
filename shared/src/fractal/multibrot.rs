use glam::DVec2;
use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn multibrot3(pos: DVec2) -> f32
{
    compute_escape_time(ITERATION_COUNT, 2.0, DVec2::ZERO, pos, |z, c|
    {
        z.comp_squared().comp_mul(&z) + c
    })
}
