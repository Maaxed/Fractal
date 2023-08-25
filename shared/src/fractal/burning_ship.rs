use glam::DVec2;
use crate::math::{Complex, Exp, ComplexNumber};
use super::escape_time_method::*;

const ITERATION_COUNT: u32 = 1024;

pub fn burning_ship(pos: Complex, params: Params) -> EscapeResult
{
    compute_escape_time(pos.conjugate(), params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        Complex::from(DVec2::from(z).abs()).squared() + c
    })
}
