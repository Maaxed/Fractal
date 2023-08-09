use crate::complex::Complex;

use super::{FractalVariation, FractalParams};

#[cfg(target_arch = "spirv")]
use num_traits::Float;

pub struct Params
{
    pub variation: FractalVariation,
    pub secondary_pos: Complex,
}

impl From<FractalParams> for Params
{
    fn from(value: FractalParams) -> Self
    {
        Params
        {
            variation: value.variation,
            secondary_pos: value.secondary_pos,
        }
    }
}

pub const DEFAULT_BAILOUT_RADIUS: f64 = 1.0e8;

pub fn compute_escape_time(pos: Complex, params: Params, iteration_count: u32, bailout_radius: f64, potential_power: Option<f32>, mut iteration_function: impl FnMut(Complex, Complex) -> Complex) -> f32
{
    let bailout_squared = bailout_radius * bailout_radius;
    let log_p = potential_power.map(|p| p.ln());
    let (mut z, c) = match params.variation
        {
            FractalVariation::Normal => (Complex::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    let mut prev_z = z;
    for i in 0..iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            return if let Some(log_p) = log_p
            {
                let log_zn = (length_squared as f32).log2() / 2.0;
                (i as f32 + 1.0 - log_zn.ln() / log_p).max(0.0)
            }
            else
            {
                i as f32
            };
        }
        z = iteration_function(z, c);

        // Periodicity checking: check for cycles with previously saved z
        if Complex::fuzzy_eq(z,  prev_z, 1.0e-20)
        {
            return -1.0;
        }

        // Save z every 32 iteration
        if i % 32 == 7
        {
            prev_z = z;
        }
    }

    -1.0
}
