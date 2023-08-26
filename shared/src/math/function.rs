use num_traits::{Float, Zero, One, Inv, Pow};
use core::ops::*;

use super::{Complex64, Complex32, Exp as ExpTrait, Trigo};


pub trait Function<I>
{
	type Output;

	fn get(&self, x: I) -> Self::Output;
}


pub trait Differentiable<I>
{
	type Derivative;

	fn derivative(&self) -> Func<Self::Derivative>;
}


#[derive(Debug, Copy, Clone)]
pub struct Func<T>(pub T);

impl<F> Func<F>
{
	pub fn get<I>(&self, x: I) -> F::Output
	where
		F: Function<I>
	{
		self.0.get(x)
	}

	pub fn derivative<I>(&self) -> Func<F::Derivative>
	where
		F: Differentiable<I>
	{
		self.0.derivative()
	}

	pub fn make(f: impl FnOnce(Func<Identity>) -> Self) -> Self
	{
		f(Func::IDENTITY)
	}

	pub fn compose<R>(self, other: Func<R>) -> Func<Composition<F, R>>
	{
		Func(Composition(self.0, other.0))
	}

	pub fn exp(self) -> Func<Composition<Exp, F>>
	{
		Func::EXP.compose(self)
	}

	pub fn ln(self) -> Func<Composition<Ln, F>>
	{
		Func::LN.compose(self)
	}

	pub fn log<B>(self, base: Func<B>) -> Func<Division<Composition<Ln, F>, Composition<Ln, B>>>
	{
		self.ln() / base.ln()
	}

	pub fn squared(self) -> Func<Composition<Squared, F>>
	{
		Func::SQUARED.compose(self)
	}

	pub fn sqrt(self) -> Func<Composition<Sqrt, F>>
	{
		Func::SQRT.compose(self)
	}

	pub fn pow_const<E>(self, exponent: E) -> Func<Composition<PowConst<E>, F>>
	{
		Func(PowConst(exponent)).compose(self)
	}

	pub fn sin(self) -> Func<Composition<Sin, F>>
	{
		Func::SIN.compose(self)
	}

	pub fn cos(self) -> Func<Composition<Cos, F>>
	{
		Func::COS.compose(self)
	}

	pub fn tan(self) -> Func<Composition<Tan, F>>
	{
		Func::TAN.compose(self)
	}
}



pub trait IntoFunc
{
	type Type;
    fn into_func(self) -> Func<Self::Type>;
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Constant<T>(pub T);

impl<T> Func<Constant<T>>
{
	pub fn constant(value: T) -> Self
	{
		Self(Constant(value))
	}
}

impl<I, O: Clone> Function<I> for Constant<O>
{
	type Output = O;

	fn get(&self, _x: I) -> Self::Output
	{
		self.0.clone()
	}
}

impl<I, O: Zero> Differentiable<I> for Constant<O>
{
	type Derivative = Constant<O>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::constant(Zero::zero())
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Identity;

impl Func<Identity>
{
	pub const IDENTITY: Self = Self(Identity);
}

impl Default for Func<Identity>
{
	fn default() -> Self
	{
		Self::IDENTITY
	}
}

impl<T> Function<T> for Identity
{
	type Output = T;
	
	fn get(&self, x: T) -> Self::Output
	{
		x
	}
}

impl<T: One> Differentiable<T> for Identity
{
	type Derivative = Constant<T>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::constant(One::one())
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Sum<A, B>(pub A, pub B);

impl<L, R: IntoFunc> Add<R> for Func<L>
{
	type Output = Func<Sum<L, R::Type>>;
	
	fn add(self, other: R) -> Self::Output
	{
		Func(Sum(self.0, other.into_func().0))
	}
}

impl<I, O, A, B> Function<I> for Sum<A, B>
where
	I: Clone,
	A: Function<I>,
	B: Function<I>,
	A::Output: Add<B::Output, Output = O>,
	{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x.clone()) + self.1.get(x)
	}
}

impl<I, A, B> Differentiable<I> for Sum<A, B>
where
	A: Differentiable<I>,
	B: Differentiable<I>
{
	type Derivative = Sum<A::Derivative, B::Derivative>;
	fn derivative(&self) -> Func<Self::Derivative>
	{
		self.0.derivative() + self.1.derivative()
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Difference<A, B>(pub A, pub B);

impl<L, R: IntoFunc> Sub<R> for Func<L>
{
	type Output = Func<Difference<L, R::Type>>;
	
	fn sub(self, other: R) -> Self::Output
	{
		Func(Difference(self.0, other.into_func().0))
	}
}

impl<I, O, A, B> Function<I> for Difference<A, B>
where
	I: Clone,
	A: Function<I>,
	B: Function<I>,
	A::Output: Sub<B::Output, Output = O>,
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x.clone()) - self.1.get(x)
	}
}

impl<I, A, B> Differentiable<I> for Difference<A, B>
where
	A: Differentiable<I>,
	B: Differentiable<I>
{
	type Derivative = Difference<A::Derivative, B::Derivative>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		self.0.derivative() - self.1.derivative()
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Negative<T>(pub T);

impl<F> Neg for Func<F>
{
	type Output = Func<Negative<F>>;
	
	fn neg(self) -> Self::Output
	{
		Func(Negative(self.0))
	}
}

impl<I, O, T> Function<I> for Negative<T>
where
	T: Function<I>,
	T::Output: Neg<Output = O>,
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		-self.0.get(x)
	}
}

impl<I, T: Differentiable<I>> Differentiable<I> for Negative<T>
{
	type Derivative = Negative<T::Derivative>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		-self.0.derivative()
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Product<A, B>(pub A, pub B);

impl<L, R: IntoFunc> Mul<R> for Func<L>
{
	type Output = Func<Product<L, R::Type>>;
	
	fn mul(self, other: R) -> Self::Output
	{
		Func(Product(self.0, other.into_func().0))
	}
}

impl<I, O, A, B> Function<I> for Product<A, B>
where
	I: Clone,
	A: Function<I>,
	B: Function<I>,
	A::Output: Mul<B::Output, Output = O>,
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x.clone()) * self.1.get(x)
	}
}

impl<I, A, B> Differentiable<I> for Product<A, B>
where
	A: Differentiable<I> + Clone,
	B: Differentiable<I> + Clone
{
	type Derivative = Sum<Product<A::Derivative, B>, Product<A, B::Derivative>>;
	fn derivative(&self) -> Func<Self::Derivative>
	{
		self.0.derivative() * Func(self.1.clone()) + Func(self.0.clone()) * self.1.derivative()
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Division<A, B>(pub A, pub B);

impl<L, R: IntoFunc> Div<R> for Func<L>
{
	type Output = Func<Division<L, R::Type>>;
	
	fn div(self, other: R) -> Self::Output
	{
		Func(Division(self.0, other.into_func().0))
	}
}

impl<I, O, A, B> Function<I> for Division<A, B>
where
	I: Clone,
	A: Function<I>,
	B: Function<I>,
	A::Output: Div<B::Output, Output = O>,
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x.clone()) / self.1.get(x)
	}
}

impl<I, A, B> Differentiable<I> for Division<A, B>
where
	A: Differentiable<I> + Clone,
	B: Differentiable<I> + Clone
{
	type Derivative = Division<Difference<Product<A::Derivative, B>, Product<A, B::Derivative>>, Product<B, B>>;
	fn derivative(&self) -> Func<Self::Derivative>
	{
		(self.0.derivative() * Func(self.1.clone()) - Func(self.0.clone()) * self.1.derivative()) / (Func(self.1.clone()) * Func(self.1.clone()))
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Inverse<T>(pub T);

impl<F> Inv for Func<F>
{
	type Output = Func<Inverse<F>>;
	
	fn inv(self) -> Self::Output
	{
		Func(Inverse(self.0))
	}
}

impl<I, O, T> Function<I> for Inverse<T>
where
	T: Function<I>,
	T::Output: Inv<Output = O>,
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x).inv()
	}
}

impl<I, T> Differentiable<I> for Inverse<T>
where
	T: Differentiable<I> + Clone
{
	type Derivative = Negative<Division<T::Derivative, Product<T, T>>>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		-(self.0.derivative() / (Func(self.0.clone()) * Func(self.0.clone())))
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Composition<A, B>(pub A, pub B); // A°B: x |-> A(B(x))

impl<I, O, A, B> Function<I> for Composition<A, B>
where
	A: Function<B::Output, Output = O>,
	B: Function<I>
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(self.1.get(x))
	}
}

impl<I, A, B> Differentiable<I> for Composition<A, B>
where
	A: Differentiable<B::Output>,
	B: Function<I> + Differentiable<I> + Clone
{
	type Derivative = Product<B::Derivative, Composition<A::Derivative, B>>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		self.1.derivative() * self.0.derivative().compose(Func(self.1.clone()))
	}
}



#[derive(Debug, Copy, Clone)]
pub struct Exp;

impl Func<Exp>
{
	pub const EXP: Self = Func(Exp);
}

impl<I: ExpTrait> Function<I> for Exp
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.exp()
	}
}

impl<I> Differentiable<I> for Exp
{
	type Derivative = Self;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func(*self)
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Ln;

impl Func<Ln>
{
	pub const LN: Self = Func(Ln);
}

impl<I: ExpTrait> Function<I> for Ln
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.ln()
	}
}

impl<I> Differentiable<I> for Ln
{
	type Derivative = Inverse<Identity>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::IDENTITY.inv()
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Squared;

impl Func<Squared>
{
	pub const SQUARED: Self = Func(Squared);
}

impl<I: ExpTrait> Function<I> for Squared
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.squared()
	}
}

impl<I> Differentiable<I> for Squared
where
	u32: Into<I>
{
	type Derivative = Product<Constant<I>, Identity>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::constant(2.into()) * Func::IDENTITY
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Sqrt;

impl Func<Sqrt>
{
	pub const SQRT: Self = Func(Sqrt);
}

impl<I: ExpTrait> Function<I> for Sqrt
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.sqrt()
	}
}

impl<I: From<f32>> Differentiable<I> for Sqrt
where
	f32: Into<I>
{
	type Derivative = Division<Constant<I>, Self>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::constant(0.5.into()) / Func(*self)
	}
}


#[derive(Debug, Copy, Clone)]
pub struct PowConst<E>(pub E);

impl<I, O, E> Function<I> for PowConst<E>
where
	I: Pow<E, Output = O>,
	E: Clone
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.pow(self.0.clone())
	}
}

impl<I, E> Differentiable<I> for PowConst<E>
	where
		E: One + Sub<Output = E> + Clone
{
	type Derivative = Product<Constant<E>, Self>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::constant(self.0.clone()) * Func(PowConst(self.0.clone() - One::one()))
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Power<B, E>(pub B, pub E);

impl<L, R> Pow<Func<R>> for Func<L>
{
	type Output = Func<Power<L, R>>;
	
	fn pow(self, other: Func<R>) -> Self::Output
	{
		Func(Power(self.0, other.0))
	}
}

impl<I, O, B, E> Function<I> for Power<B, E>
where
	I: Clone,
	B: Function<I>,
	E: Function<I>,
	B::Output: Pow<E::Output, Output = O>
{
	type Output = O;
	
	fn get(&self, x: I) -> Self::Output
	{
		self.0.get(x.clone()).pow(self.1.get(x))
	}
}

impl<I, B, E> Differentiable<I> for Power<B, E>
where
	B: Differentiable<I> + Clone,
	E: Differentiable<I> + Clone,
{
	type Derivative = Product<Self, Sum<Product<E::Derivative, Composition<Ln, B>>, Product<E, Division<B::Derivative, B>>>>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		// Using logarithmic differentiation
		// = self * (self.1 * self.0.ln()).derivative()
		Func(self.clone()) * (self.1.derivative() * Func(self.0.clone()).ln() + Func(self.1.clone()) * (self.0.derivative() / Func(self.0.clone())))
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Sin;

impl Func<Sin>
{
	pub const SIN: Self = Func(Sin);
}

impl<I: Trigo> Function<I> for Sin
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.sin()
	}
}

impl<I> Differentiable<I> for Sin
{
	type Derivative = Cos;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		Func::COS
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Cos;

impl Func<Cos>
{
	pub const COS: Self = Func(Cos);
}

impl<I: Trigo> Function<I> for Cos
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.cos()
	}
}

impl<I> Differentiable<I> for Cos
{
	type Derivative = Negative<Sin>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		-Func::SIN
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Tan;

impl Func<Tan>
{
	pub const TAN: Self = Func(Tan);
}

impl<I: Float> Function<I> for Tan
{
	type Output = I;
	
	fn get(&self, x: I) -> Self::Output
	{
		x.tan()
	}
}

impl<I> Differentiable<I> for Tan
{
	type Derivative = Inverse<Product<Cos, Cos>>;

	fn derivative(&self) -> Func<Self::Derivative>
	{
		(Func::COS * Func::COS).inv()
	}
}



impl<T> IntoFunc for Func<T>
{
	type Type = T;

	fn into_func(self) -> Func<Self::Type>
	{
		self	
	}
}

impl IntoFunc for f32
{
	type Type = Constant<f32>;
	
	fn into_func(self) -> Func<Self::Type>
	{
		Func::constant(self)
	}
}

impl IntoFunc for f64
{
	type Type = Constant<f64>;
	
	fn into_func(self) -> Func<Self::Type>
	{
		Func::constant(self)
	}
}

impl IntoFunc for Complex64
{
	type Type = Constant<Complex64>;
	
	fn into_func(self) -> Func<Self::Type>
	{
		Func::constant(self)
	}
}

impl IntoFunc for Complex32
{
	type Type = Constant<Complex32>;
	
	fn into_func(self) -> Func<Self::Type>
	{
		Func::constant(self)
	}
}
