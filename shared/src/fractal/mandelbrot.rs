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



pub fn mandelbrot_value_normal<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> EscapeResult
{
    let light_angle = 45.0;
    let light_dir = Complex::<S>::from_complex32(Complex32::from_polar(1.0, light_angle * core::f32::consts::TAU / 360.0));

    let bailout_radius: S = DEFAULT_BAILOUT_RADIUS.into();
    let bailout_squared = bailout_radius * bailout_radius;
    let (mut z, c) = match params.variation
        {
            FractalVariation::Normal => (ComplexNumber::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    
    let dc = Complex::<S>::ONE;
    let mut der = Complex::<S>::ZERO;
    for _i in 0..ITERATION_COUNT
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            let u: Complex<S> = z / der;
            let u = u / u.modulus();

            let t: f32 = u.to_vector().dot(light_dir.to_vector()).as_();
            let t = (t + 1.0) / 2.0;

            return EscapeResult::Escaped(t);
        }
        der = der * Into::<S>::into(2.0f32) * z + dc;
        z = z.squared() + c;
    }

    EscapeResult::StayedInside
}
