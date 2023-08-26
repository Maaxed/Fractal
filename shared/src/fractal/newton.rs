#![allow(clippy::needless_range_loop)]

use glam::{vec3, Vec3};
use num_traits::AsPrimitive;

use crate::math::*;

use super::{FractalParams, FractalVariation};

const ITERATION_COUNT: u32 = 128;

pub fn newton3<S: Scalar>(pos: Complex<S>, params: FractalParams<S>) -> Vec3
{
    // f(z) = z^3 - 1
    // f'(z) = 3 * z^2
    let (z, c) = match params.variation
    {
        FractalVariation::Normal => (pos, ComplexNumber::ZERO),
        FractalVariation::JuliaSet => (params.secondary_pos, pos),
    };
    newton::<S, _, 3>(ITERATION_COUNT, ComplexNumber::ONE, c, z,
        [
            ComplexNumber::from_complex32(Complex32::new(1.0, 0.0)),
            ComplexNumber::from_complex32(Complex32::new(-0.5, 3.0f32.sqrt() / 2.0)),
            ComplexNumber::from_complex32(Complex32::new(-0.5, -(3.0f32.sqrt()) / 2.0)),
        ],
        [
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        ],
    Func::make(|z| z.squared() * z - Complex::<S>::ONE),
    //|z| 3.0 * z.squared(),
    )
}

pub fn newton<S, F, const T: usize>(iteration_count: u32, a: Complex<S>, c: Complex<S>, z0: Complex<S>, roots: [Complex<S>; T], root_colors: [Vec3; T], function: Func<F>) -> Vec3
where
    S: Scalar,
    F: Function<Complex<S>, Output = Complex<S>> + Differentiable<Complex<S>>,
    F::Derivative: Function<Complex<S>, Output = Complex<S>>
{
    let derivative = function.derivative();
    let mut z = z0;

    for _i in 0..iteration_count
    {
        let delta = a * (function.get(z) / derivative.get(z)) + c;
        z -= delta;

        // Convergence checking
        if delta.fuzzy_eq(ComplexNumber::ZERO, 1.0e-16_f32.into())
        {
            break;
        }
    }

    let mut dists = [S::zero(); T];

    for i in 0..T
    {
        dists[i] = (roots[i] - z).modulus_squared().inv().min(1.0e10_f32.into());
    }

    let mut sum = S::zero();

    for i in 0..T
    {
        sum += dists[i];
    }

    let mut color = Vec3::ZERO;

    for i in 0..T
    {
        color += AsPrimitive::<f32>::as_(dists[i] / sum) * root_colors[i]
    }

    color
}