use crate::math::Complex;
use super::escape_time_method::*;
use super::FractalVariation;

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value(pos: Complex, params: Params) -> EscapeResult
{
    if params.variation == FractalVariation::Normal
    {
        // Cardioid / bulb checking
        let q = (pos + Complex::new(-0.25, 0.0)).modulus_squared();
    
        if q * (q + (pos.re() - 0.25)) <= 0.25 * pos.im() * pos.im() // the point is within the cardioid
            || (pos + Complex::new(1.0, 0.0)).modulus_squared() < 0.25 * 0.25 // the point is within the period-2 bulb
        {
            return EscapeResult::StayedInside;
        }
    }

    compute_escape_time(pos, params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        z.squared() + c
    })
}
