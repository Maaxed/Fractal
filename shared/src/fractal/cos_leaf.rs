use glam::DVec2;
use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn cos_leaf(pos: DVec2) -> f32
{
    compute_escape_time(ITERATION_COUNT, 100.0, DVec2::ZERO, pos, |z, c|
    {
        z.comp_div(&c).comp_cos()
    })
}
