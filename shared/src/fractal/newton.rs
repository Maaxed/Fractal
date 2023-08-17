#![allow(clippy::needless_range_loop)]

use glam::{vec3, Vec3};

use crate::math::*;

use super::{FractalParams, FractalVariation};

const ITERATION_COUNT: u32 = 128;

pub fn newton3(pos: Complex, params: FractalParams) -> Vec3
{
    // f(z) = z^3 - 1
    // f'(z) = 3 * z^2
    let (z, c) = match params.variation
    {
        FractalVariation::Normal => (pos, Complex::ZERO),
        FractalVariation::JuliaSet => (params.secondary_pos, pos),
    };
    newton(ITERATION_COUNT, Complex::ONE, c, z,
        [
            Complex::new(1.0, 0.0),
            Complex::new(-0.5, 3.0f64.sqrt() / 2.0),
            Complex::new(-0.5, -(3.0f64.sqrt()) / 2.0),
        ],
        [
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        ],
    Func::make(|z| z.squared() * z - Complex::ONE),
    //|z| 3.0 * z.squared(),
    )
}

pub fn newton<F, const T: usize>(iteration_count: u32, a: Complex, c: Complex, z0: Complex, roots: [Complex; T], root_colors: [Vec3; T], function: Func<F>) -> Vec3
where
    F: Function<Complex, Output = Complex> + Differentiable<Complex>,
    F::Derivative: Function<Complex, Output = Complex>
{
    let derivative = function.derivative();
    let mut z = z0;

    for _i in 0..iteration_count
    {
        let delta = a * (function.get(z) / derivative.get(z)) + c;
        z -= delta;

        // Convergence checking
        if delta.fuzzy_eq(Complex::ZERO, 1.0e-16)
        {
            break;
        }
    }

    let mut dists = [0.0; T];

    for i in 0..T
    {
        dists[i] = (1.0 / (roots[i] - z).modulus_squared()).min(1.0e10);
    }

    let mut sum: f64 = 0.0;

    for i in 0..T
    {
        sum += dists[i];
    }

    let mut color = Vec3::ZERO;

    for i in 0..T
    {
        color += (dists[i] / sum) as f32 * root_colors[i]
    }

    color
}