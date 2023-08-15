use num_traits::{Zero, One};
use core::{ops::{Add, Sub, Mul, Div, Neg}, marker::PhantomData};

use super::{Complex, Complex32};


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

	pub fn make<I>(f: impl FnOnce(Func<Identity<I>>) -> Self) -> Self
	{
		f(Func::identity())
	}

	pub fn compose<R>(self, other: Func<R>) -> Func<Composition<F, R>>
	{
		Func(Composition(self.0, other.0))
	}
}



pub trait IntoFunc
{
	type Type;
    fn into_func(self) -> Func<Self::Type>;
}



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
pub struct Identity<T>(PhantomData<fn(T) -> T>);

impl<T> Func<Identity<T>>
{
	pub fn identity() -> Self
	{
		Self(Identity(PhantomData))
	}
}

impl<T> Default for Func<Identity<T>>
{
	fn default() -> Self
	{
		Self::identity()
	}
}

impl<T> Function<T> for Identity<T>
{
	type Output = T;
	
	fn get(&self, x: T) -> Self::Output
	{
		x
	}
}

impl<T: One> Differentiable<T> for Identity<T>
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

impl<I, T> Differentiable<I> for Negative<T>
where
	T: Differentiable<I>
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

impl IntoFunc for Complex
{
	type Type = Constant<Complex>;
	
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
