use glam::DVec2;
use crate::complex::Complex;
use super::escape_time_method::compute_escape_time;

const ITERATION_COUNT: u32 = 1024;

pub fn burning_ship(pos: Complex) -> f32
{
    compute_escape_time(ITERATION_COUNT, 2.0, Complex::ZERO, pos.conjugate(), |z, c|
    {
        Complex::from(DVec2::from(z).abs()).squared() + c
    })
}
