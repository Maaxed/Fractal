use glam::{DVec2, Vec2};
use core::ops::*;
use core::iter::{Product, Sum};
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;
use num_traits::{Zero, One};

use super::{Exp, Trigo};

#[cfg(target_arch = "spirv")]
use num_traits::Float;

#[repr(C)]
#[cfg_attr(feature = "bytemuck", derive(Debug, NoUninit))]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex(DVec2);

impl From<DVec2> for Complex
{
    fn from(value: DVec2) -> Self
    {
        Complex(value)
    }
}

impl From<Complex> for DVec2
{
    fn from(value: Complex) -> Self
    {
        value.0
    }
}

impl Default for Complex
{
    fn default() -> Self
    {
        Self::ZERO
    }
}

impl Zero for Complex
{
    fn zero() -> Self
    {
        Self::ZERO
    }

    fn is_zero(&self) -> bool
    {
        *self == Self::ZERO
    }
}

impl One for Complex
{
    fn one() -> Self
    {
        Self::ONE
    }
}

impl From<f64> for Complex
{
    fn from(value: f64) -> Self
    {
        Self::new(value, 0.0)
    }
}

macro_rules! complex_from_primitive
{
    ($t:ty) =>
    {
        impl From<$t> for Complex
        {
            fn from(value: $t) -> Self
            {
                (value as f64).into()
            }
        }
    };
}

complex_from_primitive!(f32);
complex_from_primitive!(u32);
complex_from_primitive!(u64);
complex_from_primitive!(i32);
complex_from_primitive!(i64);


// Some operations are not available for f64 when compiling to spirv, so the f32 implementation is called instead
impl Complex
{
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);

    pub const fn new(real: f64, imaginary: f64) -> Self
    {
        Self(DVec2::new(real, imaginary))
    }

    pub fn from_polar(modulus: f64, argument: f64) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            Complex32::from_polar(modulus as f32, argument as f32).as_complex64()
        }
        else
        {
            Self::from(DVec2::from_angle(argument)) * modulus
        }

    }

    /// The real part of the complex number
    pub fn re(self) -> f64
    {
        self.0.x
    }

    pub fn re_mut(&mut self) -> &mut f64
    {
        &mut self.0.x
    }

    /// The imaginary part of the complex number
    pub fn im(self) -> f64
    {
        self.0.y
    }

    pub fn im_mut(&mut self) -> &mut f64
    {
        &mut self.0.y
    }

    #[doc(alias = "magnitude")]
    pub fn modulus(self) -> f64
    {
        self.0.length()
    }

    pub fn modulus_squared(self) -> f64
    {
        self.0.length_squared()
    }

    pub fn argument(self) -> f64
    {
        self.im().atan2(self.re())
    }

    pub fn conjugate(self) -> Self
    {
        Self::new(self.re(), -self.im())
    }

    pub fn as_complex32(self) -> Complex32
    {
        Complex32::from(self.0.as_vec2())
    }

    pub fn fuzzy_eq(self, rhs: Self, max_abs_diff: f64) -> bool
    {
        (self - rhs).0.abs().cmple(DVec2::splat(max_abs_diff)).all()
    }
}

impl Add<Complex> for Complex
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        (self.0 + rhs.0).into()
    }
}

impl AddAssign<Complex> for Complex
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.0 += rhs.0;
    }
}

impl Sub<Complex> for Complex
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        (self.0 - rhs.0).into()
    }
}

impl SubAssign<Complex> for Complex
{
    fn sub_assign(&mut self, rhs: Complex)
    {
        self.0 -= rhs.0;
    }
}

impl Neg for Complex
{
    type Output = Self;
    
    fn neg(self) -> Self
    {
        (-self.0).into()
    }
}

impl Mul<Complex> for Complex
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self
    {
        Self::new(self.re() * rhs.re() - self.im() * rhs.im(), self.re() * rhs.im() + self.im() * rhs.re())
    }
}

impl MulAssign<Complex> for Complex
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Complex
{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self
    {
        (self.0 * rhs).into()
    }
}

impl MulAssign<f64> for Complex
{
    fn mul_assign(&mut self, rhs: f64)
    {
        self.0 *= rhs;
    }
}

impl Mul<Complex> for f64
{
    type Output = Complex;
    
    fn mul(self, rhs: Complex) -> Complex
    {
        (self * rhs.0).into()
    }
}

impl Div<Complex> for Complex
{
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self
    {
        self * rhs.conjugate() / rhs.modulus_squared()
    }
}

impl DivAssign<Complex> for Complex
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = *self / rhs
    }
}

impl Div<f64> for Complex
{
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self
    {
        (self.0 / rhs).into()
    }
}

impl DivAssign<f64> for Complex
{
    fn div_assign(&mut self, rhs: f64)
    {
        self.0 /= rhs;
    }
}

impl Sum for Complex
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Complex
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| a + b)
    }
}

impl Product for Complex
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Complex
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| a * b)
    }
}

impl Exp for Complex
{
    fn squared(self) -> Self
    {
        Self::new(self.re() * self.re() - self.im() * self.im(), 2.0 * self.re() * self.im())
    }

    fn sqrt(self) -> Self
    {
        // In polar coordinate: Self::from_polar(modulus.sqrt(), arg / 2.0)

        let modulus = self.modulus();
        let sgn = if self.im() < 0.0 { -1.0 } else { 1.0 };
        Self::new((modulus + self.re()) / 2.0, sgn * (modulus - self.re()) / 2.0)
    }

    fn exp(self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_complex32().exp().as_complex64()
        }
        else
        {
            Self::from_polar(Exp::exp(self.re()), self.im())
        }
    }

    fn pow(self, exp: Self) -> Self
    {
        (exp * self.ln()).exp()
    }

    fn ln(self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_complex32().ln().as_complex64()
        }
        else
        {
            Self::new(Exp::ln(self.modulus()), self.argument())
        }
    }

    fn log(self, base: Self) -> Self
    {
        self.ln() / base.ln()
    }
}

impl super::Trigo for Complex
{
    fn sin(self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_complex32().sin().as_complex64()
        }
        else
        {
            let (sin, cos) = self.re().sin_cos();
            Self::new(sin * self.im().cosh(), cos * self.im().sinh())
        }
    }
    
    fn cos(self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_complex32().cos().as_complex64()
        }
        else
        {
            let (sin, cos) = self.re().sin_cos();
            Self::new(cos * self.im().cosh(), -sin * self.im().sinh())
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex32(Vec2);

impl From<Vec2> for Complex32
{
    fn from(value: Vec2) -> Self
    {
        Complex32(value)
    }
}

impl From<Complex32> for Vec2
{
    fn from(value: Complex32) -> Self
    {
        value.0
    }
}

impl Default for Complex32
{
    fn default() -> Self
    {
        Self::ZERO
    }
}

impl From<f32> for Complex32
{
    fn from(value: f32) -> Self
    {
        Self::new(value, 0.0)
    }
}

macro_rules! complex_from_primitive
{
    ($t:ty) =>
    {
        impl From<$t> for Complex32
        {
            fn from(value: $t) -> Self
            {
                (value as f32).into()
            }
        }
    };
}

complex_from_primitive!(f64);
complex_from_primitive!(u32);
complex_from_primitive!(u64);
complex_from_primitive!(i32);
complex_from_primitive!(i64);

impl Complex32
{
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);

    pub const fn new(real: f32, imaginary: f32) -> Self
    {
        Self(Vec2::new(real, imaginary))
    }

    pub fn from_polar(modulus: f32, argument: f32) -> Self
    {
        Self::from(Vec2::from_angle(argument)) * modulus
    }

    /// The real part of the complex number
    pub fn re(self) -> f32
    {
        self.0.x
    }

    /// The imaginary part of the complex number
    pub fn im(self) -> f32
    {
        self.0.y
    }

    #[doc(alias = "magnitude")]
    pub fn modulus(self) -> f32
    {
        self.0.length()
    }

    pub fn modulus_squared(self) -> f32
    {
        self.0.length_squared()
    }

    pub fn argument(self) -> f32
    {
        self.im().atan2(self.re())
    }

    pub fn conjugate(self) -> Self
    {
        Self::new(self.re(), -self.im())
    }

    pub fn as_complex64(self) -> Complex
    {
        Complex::from(self.0.as_dvec2())
    }

    pub fn fuzzy_eq(self, rhs: Self, max_abs_diff: f32) -> bool
    {
        (self - rhs).0.abs().cmple(Vec2::splat(max_abs_diff)).all()
    }
}

impl super::Exp for Complex32
{
    fn squared(self) -> Self
    {
        Self::new(self.re() * self.re() - self.im() * self.im(), 2.0 * self.re() * self.im())
    }

    fn sqrt(self) -> Self
    {
        // In polar coordinate: Self::from_polar(modulus.sqrt(), arg / 2.0)

        let modulus = self.modulus();
        let sgn = if self.im() < 0.0 { -1.0 } else { 1.0 };
        Self::new((modulus + self.re()) / 2.0, sgn * (modulus - self.re()) / 2.0)
    }

    fn exp(self) -> Self
    {
        Self::from_polar(Exp::exp(self.re()), self.im())
    }

    fn pow(self, exp: Self) -> Self
    {
        (exp * self.ln()).exp()
    }

    fn ln(self) -> Self
    {
        Self::new(Exp::ln(self.modulus()), self.argument())
    }

    fn log(self, base: Self) -> Self
    {
        self.ln() / base.ln()
    }
}

impl Add<Complex32> for Complex32
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        (self.0 + rhs.0).into()
    }
}

impl AddAssign<Complex32> for Complex32
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.0 += rhs.0;
    }
}

impl Sub<Complex32> for Complex32
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        (self.0 - rhs.0).into()
    }
}

impl SubAssign<Complex32> for Complex32
{
    fn sub_assign(&mut self, rhs: Complex32)
    {
        self.0 -= rhs.0;
    }
}

impl Neg for Complex32
{
    type Output = Self;
    
    fn neg(self) -> Self
    {
        (-self.0).into()
    }
}

impl Mul<Complex32> for Complex32
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self
    {
        Self::new(self.re() * rhs.re() - self.im() * rhs.im(), self.re() * rhs.im() + self.im() * rhs.re())
    }
}

impl MulAssign<Complex32> for Complex32
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Complex32
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self
    {
        (self.0 * rhs).into()
    }
}

impl MulAssign<f32> for Complex32
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.0 *= rhs;
    }
}

impl Mul<Complex32> for f32
{
    type Output = Complex32;
    
    fn mul(self, rhs: Complex32) -> Complex32
    {
        (self * rhs.0).into()
    }
}

impl Div<Complex32> for Complex32
{
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self
    {
        self * rhs.conjugate() / rhs.modulus_squared()
    }
}

impl DivAssign<Complex32> for Complex32
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = *self / rhs
    }
}

impl Div<f32> for Complex32
{
    type Output = Self;
    
    fn div(self, rhs: f32) -> Self
    {
        (self.0 / rhs).into()
    }
}

impl DivAssign<f32> for Complex32
{
    fn div_assign(&mut self, rhs: f32)
    {
        self.0 /= rhs;
    }
}

impl Sum for Complex32
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Complex32
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| a + b)
    }
}

impl Product for Complex32
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Complex32
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| a * b)
    }
}

impl Trigo for Complex32
{
    fn sin(self) -> Self
    {
        let (sin, cos) = self.re().sin_cos();
        Self::new(sin * self.im().cosh(), cos * self.im().sinh())
    }
    
    fn cos(self) -> Self
    {
        let (sin, cos) = self.re().sin_cos();
        Self::new(cos * self.im().cosh(), -sin * self.im().sinh())
    }
}