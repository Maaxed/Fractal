use glam::DVec2;
use crate::complex::Complex;
use super::escape_time_method::{compute_escape_time, Params};

const ITERATION_COUNT: u32 = 1024;

pub fn burning_ship(pos: Complex, params: Params) -> f32
{
    compute_escape_time(pos.conjugate(), params, ITERATION_COUNT, 2.0, |z, c|
    {
        Complex::from(DVec2::from(z).abs()).squared() + c
    })
}
