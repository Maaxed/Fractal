use crate::complex::Complex;

use super::{FractalVariation, FractalParams};

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

pub fn compute_escape_time(pos: Complex, params: Params, iteration_count: u32, max_length: f64, mut iteration_function: impl FnMut(Complex, Complex) -> Complex) -> f32
{
    let max_length_squared = max_length * max_length;
    let (mut z, c) = match params.variation
        {
            FractalVariation::Normal => (Complex::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    let mut prev_z = z;
    for i in 0..iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > max_length_squared
        {
            return i as f32 / iteration_count as f32;
        }
        z = iteration_function(z, c);

        // Periodicity checking: check for cycles with previously saved z
        if Complex::fuzzy_eq(z,  prev_z, 1.0e-20)
        {
            return 1.0;
        }

        // Save z every 32 iteration
        if i % 32 == 7
        {
            prev_z = z;
        }
    }

    1.0
}
