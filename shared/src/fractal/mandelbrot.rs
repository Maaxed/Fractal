use glam::DVec2;

use crate::math::*;
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



pub fn mandelbrot_value_normal(pos: Complex, params: Params) -> EscapeResult
{
    let light_angle = 45.0;
    let light_dir = Complex::from_polar(1.0, light_angle * core::f64::consts::TAU / 360.0);

    let bailout_squared = DEFAULT_BAILOUT_RADIUS * DEFAULT_BAILOUT_RADIUS;
    let (mut z, c) = match params.variation
        {
            FractalVariation::Normal => (Complex::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    
    let dc = Complex::ONE;
    let mut der = Complex::ZERO;
    for _i in 0..ITERATION_COUNT
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            let u = z / der;
            let u = u / u.modulus();

            let t = DVec2::from(u).dot(DVec2::from(light_dir));
            let t = (t + 1.0) / 2.0;

            return EscapeResult::Escaped(t as f32);
        }
        der = der * 2.0 * z + dc;
        z = z.squared() + c;
    }

    EscapeResult::StayedInside
}
