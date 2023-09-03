use core::ops::*;
use core::iter::{Product, Sum};
#[cfg(feature = "bytemuck")]
use bytemuck::NoUninit;
use num_traits::{Zero, One, Inv};

use super::function::{IntoFunc, Constant};
use super::{Exp, Trigo, Scalar, Vec2};

#[cfg(target_arch = "spirv")]
use num_traits::Float;

pub trait ComplexNumber:
    Copy
    + PartialEq
    + Add<Output = Self> + AddAssign
    + Sub<Output = Self> + SubAssign
    + Mul<Output = Self> + MulAssign
    + Div<Output = Self> + DivAssign
    + Mul<Self::Scalar, Output = Self> + MulAssign<Self::Scalar>
    + Div<Self::Scalar, Output = Self> + DivAssign<Self::Scalar>
    + Neg<Output = Self>
    + Inv<Output = Self>
    + Zero + One
    + From<f32> + From<f64>
    + From<u32> + From<u64>
    + From<i32> + From<i64>
    + Exp
    + Trigo
    + IntoFunc<Type = Constant<Self>>
{
    type Scalar: Scalar<Complex = Self>;

    const ZERO: Self;
    const ONE: Self;
    const I: Self;

    fn from_cartesian(real: Self::Scalar, imaginary: Self::Scalar) -> Self;

    fn from_polar(modulus: Self::Scalar, argument: Self::Scalar) -> Self;

    fn from_vector(vec: Vec2<Self::Scalar>) -> Self;

    fn to_vector(self) -> Vec2<Self::Scalar>;

    /// The real part of the complex number
    fn re(self) -> Self::Scalar;

    fn re_mut(&mut self) -> &mut Self::Scalar;

    /// The imaginary part of the complex number
    fn im(self) -> Self::Scalar;

    fn im_mut(&mut self) -> &mut Self::Scalar;

    #[doc(alias = "magnitude")]
    fn modulus(self) -> Self::Scalar;

    fn modulus_squared(self) -> Self::Scalar;

    fn argument(self) -> Self::Scalar;

    fn conjugate(self) -> Self;

    fn fuzzy_eq(self, rhs: Self, max_abs_diff: Self::Scalar) -> bool;
    
    fn from_complex32(value: Complex32) -> Self;

    fn to_complex32(self) -> Complex32;
    
    fn to_complex64(self) -> Complex64;
}

#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex64(f64, f64);

impl Default for Complex64
{
    fn default() -> Self
    {
        Self::ZERO
    }
}

impl Zero for Complex64
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

impl One for Complex64
{
    fn one() -> Self
    {
        Self::ONE
    }
}

impl From<f64> for Complex64
{
    fn from(value: f64) -> Self
    {
        Self::new(value, 0.0)
    }
}

macro_rules! complex64_from_primitive
{
    ($t:ty) =>
    {
        impl From<$t> for Complex64
        {
            fn from(value: $t) -> Self
            {
                (value as f64).into()
            }
        }
    };
}

complex64_from_primitive!(f32);
complex64_from_primitive!(u32);
complex64_from_primitive!(u64);
complex64_from_primitive!(i32);
complex64_from_primitive!(i64);


// Some operations are not available for f64 when compiling to spirv, so the f32 implementation is called instead
impl Complex64
{
    pub const fn new(real: f64, imaginary: f64) -> Self
    {
        Self(real, imaginary)
    }
    
    fn to_vector_mut(&mut self) -> &mut Vec2<f64>
    {
        unsafe { &mut *(self as *mut Self as *mut Vec2<f64>) }
    }
}

impl ComplexNumber for Complex64
{
    type Scalar = f64;

    const ZERO: Self = Self::new(0.0, 0.0);
    const ONE: Self = Self::new(1.0, 0.0);
    const I: Self = Self::new(0.0, 1.0);
    
    fn from_cartesian(real: f64, imaginary: f64) -> Self
    {
        Self::new(real, imaginary)
    }

    fn from_polar(modulus: f64, argument: f64) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            Complex32::from_polar(modulus as f32, argument as f32).to_complex64()
        }
        else
        {
            Self::from_vector(Vec2::<Self::Scalar>::from_angle(argument)) * modulus
        }

    }
    
    fn from_vector(vec: Vec2<Self::Scalar>) -> Self
    {
        Self::new(vec.x, vec.y)
    }

    fn to_vector(self) -> Vec2<Self::Scalar>
    {
        Vec2::<Self::Scalar>::new(self.re(), self.im())
    }

    /// The real part of the complex number
    fn re(self) -> f64
    {
        self.0
    }

    fn re_mut(&mut self) -> &mut f64
    {
        &mut self.0
    }

    /// The imaginary part of the complex number
    fn im(self) -> f64
    {
        self.1
    }

    fn im_mut(&mut self) -> &mut f64
    {
        &mut self.1
    }

    #[doc(alias = "magnitude")]
    fn modulus(self) -> f64
    {
        self.to_vector().length()
    }

    fn modulus_squared(self) -> f64
    {
        self.to_vector().length_squared()
    }

    fn argument(self) -> f64
    {
        
        if cfg!(target_arch = "spirv")
        {
            self.to_complex32().argument() as f64
        }
        else
        {
            self.im().atan2(self.re())
        }
    }

    fn conjugate(self) -> Self
    {
        Self::new(self.re(), -self.im())
    }

    fn fuzzy_eq(self, rhs: Self, max_abs_diff: f64) -> bool
    {
        (self - rhs).to_vector().abs().cmple(Vec2::<Self::Scalar>::splat(max_abs_diff)).all()
    }
    
    fn from_complex32(value: Complex32) -> Self
    {
        value.to_complex64()
    }
    
    fn to_complex32(self) -> Complex32
    {
        Complex32::from_vector(self.to_vector().as_vec2())
    }
    
    fn to_complex64(self) -> Self
    {
        self
    }
}

impl Add<Complex64> for Complex64
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        Self::from_vector(self.to_vector() + rhs.to_vector())
    }
}

impl AddAssign<Complex64> for Complex64
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self.to_vector_mut() += rhs.to_vector();
    }
}

impl Sub<Complex64> for Complex64
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        Self::from_vector(self.to_vector() - rhs.to_vector())
    }
}

impl SubAssign<Complex64> for Complex64
{
    fn sub_assign(&mut self, rhs: Complex64)
    {
        *self.to_vector_mut() -= rhs.to_vector();
    }
}

impl Neg for Complex64
{
    type Output = Self;
    
    fn neg(self) -> Self
    {
        Self::from_vector(-self.to_vector())
    }
}

impl Mul<Complex64> for Complex64
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self
    {
        Self::new(self.re() * rhs.re() - self.im() * rhs.im(), self.re() * rhs.im() + self.im() * rhs.re())
    }
}

impl MulAssign<Complex64> for Complex64
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Complex64
{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self
    {
        Self::from_vector(self.to_vector() * rhs)
    }
}

impl MulAssign<f64> for Complex64
{
    fn mul_assign(&mut self, rhs: f64)
    {
        *self.to_vector_mut() *= rhs;
    }
}

impl Mul<Complex64> for f64
{
    type Output = Complex64;
    
    fn mul(self, rhs: Complex64) -> Complex64
    {
        Complex64::from_vector(self * rhs.to_vector())
    }
}

impl Div<Complex64> for Complex64
{
    type Output = Self;
    
    fn div(self, rhs: Self) -> Self
    {
        self * rhs.conjugate() / rhs.modulus_squared()
    }
}

impl DivAssign<Complex64> for Complex64
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = *self / rhs
    }
}

impl Div<f64> for Complex64
{
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self
    {
        Self::from_vector(self.to_vector() / rhs)
    }
}

impl DivAssign<f64> for Complex64
{
    fn div_assign(&mut self, rhs: f64)
    {
        *self.to_vector_mut() /= rhs;
    }
}

impl Inv for Complex64
{
    type Output = Self;

    fn inv(self) -> Self::Output
    {
        self.conjugate() / self.modulus_squared()
    }
}

impl Sum for Complex64
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Complex64
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| a + b)
    }
}

impl Product for Complex64
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Complex64
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| a * b)
    }
}

impl Exp for Complex64
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
            self.to_complex32().exp().to_complex64()
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
            self.to_complex32().ln().to_complex64()
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

impl super::Trigo for Complex64
{
    fn sin(self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.to_complex32().sin().to_complex64()
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
            self.to_complex32().cos().to_complex64()
        }
        else
        {
            let (sin, cos) = self.re().sin_cos();
            Self::new(cos * self.im().cosh(), -sin * self.im().sinh())
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
#[cfg_attr(feature = "bytemuck", derive(NoUninit))]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex32(f32, f32);

impl Default for Complex32
{
    fn default() -> Self
    {
        Self::ZERO
    }
}

impl Zero for Complex32
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

impl One for Complex32
{
    fn one() -> Self
    {
        Self::ONE
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
    pub const fn new(real: f32, imaginary: f32) -> Self
    {
        Self(real, imaginary)
    }
    
    fn to_vector_mut(&mut self) -> &mut Vec2<f32>
    {
        unsafe { &mut *(self as *mut Self as *mut Vec2<f32>) }
    }
}

impl ComplexNumber for Complex32
{
    type Scalar = f32;

    const ZERO: Self = Self::new(0.0, 0.0);
    const ONE: Self = Self::new(1.0, 0.0);
    const I: Self = Self::new(0.0, 1.0);

    fn from_cartesian(real: f32, imaginary: f32) -> Self
    {
        Self::new(real, imaginary)
    }

    fn from_polar(modulus: f32, argument: f32) -> Self
    {
        Self::from_vector(Vec2::<Self::Scalar>::from_angle(argument)) * modulus
    }
    
    fn from_vector(vec: Vec2<Self::Scalar>) -> Self
    {
        Self::new(vec.x, vec.y)
    }

    fn to_vector(self) -> Vec2<Self::Scalar>
    {
        Vec2::<Self::Scalar>::new(self.re(), self.im())
    }

    /// The real part of the complex number
    fn re(self) -> f32
    {
        self.0
    }

    fn re_mut(&mut self) -> &mut f32
    {
        &mut self.0
    }

    /// The imaginary part of the complex number
    fn im(self) -> f32
    {
        self.1
    }

    fn im_mut(&mut self) -> &mut f32
    {
        &mut self.1
    }

    #[doc(alias = "magnitude")]
    fn modulus(self) -> f32
    {
        self.to_vector().length()
    }

    fn modulus_squared(self) -> f32
    {
        self.to_vector().length_squared()
    }

    fn argument(self) -> f32
    {
        self.im().atan2(self.re())
    }

    fn conjugate(self) -> Self
    {
        Self::new(self.re(), -self.im())
    }

    fn fuzzy_eq(self, rhs: Self, max_abs_diff: f32) -> bool
    {
        (self - rhs).to_vector().abs().cmple(Vec2::<Self::Scalar>::splat(max_abs_diff)).all()
    }
    
    fn from_complex32(value: Complex32) -> Self
    {
        value
    }

    fn to_complex32(self) -> Complex32
    {
        self
    }

    fn to_complex64(self) -> Complex64
    {
        Complex64::from_vector(self.to_vector().as_dvec2())
    }
}

impl Add<Complex32> for Complex32
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        Self::from_vector(self.to_vector() + rhs.to_vector())
    }
}

impl AddAssign<Complex32> for Complex32
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self.to_vector_mut() += rhs.to_vector();
    }
}

impl Sub<Complex32> for Complex32
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self
    {
        Self::from_vector(self.to_vector() - rhs.to_vector())
    }
}

impl SubAssign<Complex32> for Complex32
{
    fn sub_assign(&mut self, rhs: Complex32)
    {
        *self.to_vector_mut() -= rhs.to_vector();
    }
}

impl Neg for Complex32
{
    type Output = Self;
    
    fn neg(self) -> Self
    {
        Self::from_vector(-self.to_vector())
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
        Self::from_vector(self.to_vector() * rhs)
    }
}

impl MulAssign<f32> for Complex32
{
    fn mul_assign(&mut self, rhs: f32)
    {
        *self.to_vector_mut() *= rhs;
    }
}

impl Mul<Complex32> for f32
{
    type Output = Complex32;
    
    fn mul(self, rhs: Complex32) -> Complex32
    {
        Complex32::from_vector(self * rhs.to_vector())
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
        Self::from_vector(self.to_vector() / rhs)
    }
}

impl DivAssign<f32> for Complex32
{
    fn div_assign(&mut self, rhs: f32)
    {
        *self.to_vector_mut() /= rhs;
    }
}

impl Inv for Complex32
{
    type Output = Self;

    fn inv(self) -> Self::Output
    {
        self.conjugate() / self.modulus_squared()
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
