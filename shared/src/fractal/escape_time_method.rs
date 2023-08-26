use crate::math::*;

use super::{FractalVariation, FractalParams};


pub const DEFAULT_BAILOUT_RADIUS: f32 = 1.0e8;

pub enum EscapeResult
{
    Escaped(f32),
    StayedInside
}

pub fn compute_escape_time<S: Scalar>(pos: Complex<S>, params: FractalParams<S>, iteration_count: u32, bailout_radius: f32, potential_power: Option<f32>, mut iteration_function: impl FnMut(Complex<S>, Complex<S>) -> Complex<S>) -> EscapeResult
{
    let bailout_radius: S = bailout_radius.into();
    let bailout_squared = bailout_radius * bailout_radius;
    let log_p = potential_power.map(ln);
    let (mut z, c) = match params.variation
        {
            FractalVariation::Normal => (ComplexNumber::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    let mut prev_z = z;
    for i in 0..iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            return EscapeResult::Escaped(if let Some(log_p) = log_p
            {
                let log_zn = log2(length_squared.as_()) / 2.0;
                (i as f32 + 1.0 - ln(log_zn) / log_p).max(0.0)
            }
            else
            {
                i as f32
            });
        }
        z = iteration_function(z, c);

        // Periodicity checking: check for cycles with previously saved z
        if ComplexNumber::fuzzy_eq(z,  prev_z, 1.0e-20_f32.into())
        {
            return EscapeResult::StayedInside;
        }

        // Save z every 32 iteration
        if i % 32 == 7
        {
            prev_z = z;
        }
    }

    EscapeResult::StayedInside
}
