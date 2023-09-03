use crate::math::*;
use super::FractalParams;
use super::RenderTechnique;
use super::escape_time_method::*;
use super::FractalVariation;

const ITERATION_COUNT: u32 = 1024;

pub fn mandelbrot_value<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    if params.variation == FractalVariation::Normal && params.render_technique == RenderTechnique::Normal
    {
        // Cardioid / bulb checking
        let quarter: S = 0.25_f32.into();
        let q = (pos + ComplexNumber::from_cartesian(-quarter, S::zero())).modulus_squared();
    
        if q * (q + (pos.re() - quarter)) <= quarter * pos.im() * pos.im() // the point is within the cardioid
            || (pos + ComplexNumber::ONE).modulus_squared() < quarter * quarter // the point is within the period-2 bulb
        {
            return EscapeResult::StayedInside;
        }
    }

    compute_escape_time_fractal(pos, params, ITERATION_COUNT, DEFAULT_BAILOUT_RADIUS, Some(2.0), |z, c|
    {
        z.squared() + c
    })
}
