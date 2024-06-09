pub use core::cell::Cell;
use core::marker::PhantomData;

use crate::math::*;

use super::{FractalVariation, FractalParams};


pub const DEFAULT_BAILOUT_RADIUS: f32 = 1.0e8;

pub enum EscapeResult
{
    Escaped(f32),
    StayedInside
}

#[derive(Clone, Copy)]
pub struct Z<S: Scalar>(PhantomData<S>);
#[derive(Clone, Copy)]
pub struct DZ<S: Scalar>(PhantomData<S>);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C<S: Scalar>(Complex<S>);
#[derive(Clone, Copy)]
pub struct DC<S: Scalar>(PhantomData<S>);

impl<C: ComplexNumber> Function<C> for Z<C::Scalar>
{
    type Output = C;

    fn get(&self, z: C) -> Self::Output
    {
        z
    }
}

impl<CN: ComplexNumber> Function<CN> for C<CN::Scalar>
{
    type Output = CN;

    fn get(&self, _z: CN) -> Self::Output
    {
        self.0
    }
}

impl<S: Scalar> Function<(Complex<S>, Complex<S>)> for Z<S>
{
    type Output = Complex<S>;

    fn get(&self, (z, _dz): (Complex<S>, Complex<S>)) -> Self::Output
    {
        z
    }
}

impl<S: Scalar> Function<(Complex<S>, Complex<S>)> for C<S>
{
    type Output = Complex<S>;

    fn get(&self, (_z, _dz): (Complex<S>, Complex<S>)) -> Self::Output
    {
        self.0
    }
}

impl<S: Scalar> Differentiable<(Complex<S>, Complex<S>)> for Z<S>
{
    type Derivative = DZ<S>;

    fn derivative(&self) -> Func<Self::Derivative>
    {
        Func(DZ(PhantomData))
    }
}

impl<S: Scalar> Function<(Complex<S>, Complex<S>)> for DZ<S>
{
    type Output = Complex<S>;

    fn get(&self, (_z, dz): (Complex<S>, Complex<S>)) -> Self::Output
    {
        dz
    }
}

impl<S: Scalar> Differentiable<(Complex<S>, Complex<S>)> for C<S>
{
    type Derivative = DC<S>;

    fn derivative(&self) -> Func<Self::Derivative>
    {
        Func(DC(PhantomData))
    }
}

impl<S: Scalar> Function<(Complex<S>, Complex<S>)> for DC<S>
{
    type Output = Complex<S>;

    fn get(&self, (_z, _dz): (Complex<S>, Complex<S>)) -> Self::Output
    {
        Self::Output::ONE
    }
}

fn partial_apply<S, F, IF>(f: F, c: Complex<S>) -> impl FnMut(Complex<S>) -> Complex<S> 
where
    S: Scalar,
    F: FnOnce(Func<Z<S>>, Func<C<S>>) -> Func<IF>,
    IF: Function<(Complex<S>, Complex<S>), Output = Complex<S>> + Differentiable<(Complex<S>, Complex<S>)>,
    IF::Derivative: Function<(Complex<S>, Complex<S>), Output = Complex<S>>,
{
    let fun = f(Func(Z(PhantomData)), Func(C(c)));
    move |z| fun.get((z, Complex::<S>::ZERO))
}

pub fn compute_escape_time_fractal<S, F, IF>(pos: Complex<S>, params: FractalParams<S>, bailout_radius: f32, potential_power: Option<f32>, iteration_function: F) -> EscapeResult
where
    S: Scalar,
    F: FnOnce(Func<Z<S>>, Func<C<S>>) -> Func<IF>,
    IF: Function<(Complex<S>, Complex<S>), Output = Complex<S>> + Differentiable<(Complex<S>, Complex<S>)>,
    IF::Derivative: Function<(Complex<S>, Complex<S>), Output = Complex<S>>,
{
    let (z, c) = match params.variation
        {
            FractalVariation::Normal => (ComplexNumber::ZERO, pos),
            FractalVariation::JuliaSet => (pos, params.secondary_pos),
        };
    
    match params.render_technique
    {
        super::RenderTechnique::Normal =>
            compute_escape_time::<S>(z, params.iteration_limit, bailout_radius, potential_power, partial_apply::<S, _, _>(iteration_function, c)),
        super::RenderTechnique::OrbitTrapPoint =>
            EscapeResult::Escaped(compute_orbit_trap::<S>(z, params.iteration_limit, partial_apply::<S, _, _>(iteration_function, c), |z| z.modulus_squared())),
        super::RenderTechnique::OrbitTrapCross =>
            EscapeResult::Escaped(compute_orbit_trap::<S>(z, params.iteration_limit, partial_apply::<S, _, _>(iteration_function, c), |z| z.re().abs().min(z.im().abs()))),
        super::RenderTechnique::NormalMap =>
            compute_normal_map::<S, _, _>(z, c, params.iteration_limit, bailout_radius, iteration_function),
    }
}

pub fn compute_escape_time<S: Scalar>(mut z: Complex<S>, iteration_count: u32, bailout_radius: f32, potential_power: Option<f32>, mut iteration_function: impl FnMut(Complex<S>) -> Complex<S>) -> EscapeResult
{
    let bailout_radius: S = bailout_radius.into();
    let bailout_squared = bailout_radius * bailout_radius;
    let log_p = potential_power.map(ln);
    let mut prev_z = z;
    for i in 1..=iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            return EscapeResult::Escaped(if let Some(log_p) = log_p
            {
                let log_zn = log2(length_squared.as_()) / 2.0;
                (i as f32 + 1.0 - ln(log_zn) / log_p).max(1.0)
            }
            else
            {
                i as f32
            });
        }
        z = iteration_function(z);

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

pub fn compute_orbit_trap<S: Scalar>(mut z: Complex<S>, iteration_count: u32, mut iteration_function: impl FnMut(Complex<S>) -> Complex<S>, mut distance_function: impl FnMut(Complex<S>) -> S) -> f32
{
    let mut dist = S::max_value();

    for _i in 0..iteration_count
    {
        z = iteration_function(z);
        dist = dist.min(distance_function(z));
    }

    dist.as_()
}

pub fn compute_normal_map<S, F, IF>(mut z: Complex<S>, c: Complex<S>, iteration_count: u32, bailout_radius: f32, iteration_function: F) -> EscapeResult
where
    S: Scalar,
    F: FnOnce(Func<Z<S>>, Func<C<S>>) -> Func<IF>,
    IF: Function<(Complex<S>, Complex<S>), Output = Complex<S>> + Differentiable<(Complex<S>, Complex<S>)>,
    IF::Derivative: Function<(Complex<S>, Complex<S>), Output = Complex<S>>,
{
    let light_angle = 45.0;
    let light_dir = Complex::<S>::from_complex32(Complex32::from_polar(1.0, light_angle * core::f32::consts::TAU / 360.0));

    let bailout_radius: S = bailout_radius.into();
    let bailout_squared = bailout_radius * bailout_radius;

    let iter_fn = iteration_function(Func(Z(PhantomData)), Func(C(c)));
    let derivative = iter_fn.derivative();

    let mut dz = Complex::<S>::ZERO;
    for _i in 0..iteration_count
    {
        let length_squared = z.modulus_squared();
        if length_squared > bailout_squared
        {
            let u: Complex<S> = z / dz;
            let u = u / u.modulus();

            let t: f32 = u.to_vector().dot(light_dir.to_vector()).as_();
            let t = (t + 1.0) / 2.0;

            return EscapeResult::Escaped(t);
        }
        dz = derivative.get((z, dz));
        z = iter_fn.get((z, dz));
    }

    EscapeResult::StayedInside
}
