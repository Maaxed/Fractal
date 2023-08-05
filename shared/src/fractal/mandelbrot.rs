use crate::complex::Complex;
use super::{escape_time_method::{compute_escape_time, Params}, FractalVariation};

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value(pos: Complex, params: Params) -> f32
{
    if params.variation == FractalVariation::Normal
    {
        // Cardioid / bulb checking
        let q = (pos + Complex::new(-0.25, 0.0)).modulus_squared();
    
        if q * (q + (pos.re() - 0.25)) <= 0.25 * pos.im() * pos.im() // the point is within the cardioid
            || (pos + Complex::new(1.0, 0.0)).modulus_squared() < 0.25 * 0.25 // the point is within the period-2 bulb
        {
            return 1.0
        }
    }

    compute_escape_time(pos, params, ITERATION_COUNT, 2.0, |z, c|
    {
        z.squared() + c
    })
}
