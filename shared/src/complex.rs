use glam::{DVec2, Vec2};

#[cfg(target_arch = "spirv")]
use num_traits::Float;

pub trait Complex
{
    fn comp_conjugate(&self) -> Self;
    fn comp_mul(&self, rhs: &Self) -> Self;
    fn comp_div(&self, rhs: &Self) -> Self;
    fn comp_squared(&self) -> Self;
    fn comp_exp(&self) -> Self;
    fn comp_ln(&self) -> Self;
    fn comp_sin(&self) -> Self;
    fn comp_cos(&self) -> Self;
}

// Some operations are not available for f64 when compiling to spirv, so the f32 implementation is called instead
impl Complex for DVec2
{
    fn comp_conjugate(&self) -> Self
    {
        Self::new(self.x, -self.y)
    }

    fn comp_mul(&self, rhs: &Self) -> Self
    {
        Self::new(self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x)
    }

    fn comp_div(&self, rhs: &Self) -> Self
    {
        self.comp_mul(&rhs.comp_conjugate()) / rhs.length_squared()
    }

    fn comp_squared(&self) -> Self
    {
        Self::new(self.x * self.x - self.y * self.y, 2.0 * self.x * self.y)
    }

    fn comp_exp(&self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_vec2().comp_exp().as_dvec2()
        }
        else
        {
            Self::from_angle(self.y) * self.x.exp()
        }
    }

    fn comp_ln(&self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_vec2().comp_ln().as_dvec2()
        }
        else
        {
            Self::new(self.length().ln(), self.y.atan2(self.x))
        }
    }
    
    fn comp_sin(&self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_vec2().comp_sin().as_dvec2()
        }
        else
        {
            Self::new(self.x.sin() * self.y.cosh(), self.x.cos() * self.y.sinh())
        }
    }
    
    fn comp_cos(&self) -> Self
    {
        if cfg!(target_arch = "spirv")
        {
            self.as_vec2().comp_cos().as_dvec2()
        }
        else
        {
            Self::new(self.x.cos() * self.y.cosh(), -self.x.sin() * self.y.sinh())
        }
    }
}

impl Complex for Vec2
{
    fn comp_conjugate(&self) -> Self
    {
        Self::new(self.x, -self.y)
    }

    fn comp_mul(&self, rhs: &Self) -> Self
    {
        Self::new(self.x * rhs.x - self.y * rhs.y, self.x * rhs.y + self.y * rhs.x)
    }

    fn comp_div(&self, rhs: &Self) -> Self
    {
        self.comp_mul(&rhs.comp_conjugate()) / rhs.length_squared()
    }

    fn comp_squared(&self) -> Self
    {
        Self::new(self.x * self.x - self.y * self.y, 2.0 * self.x * self.y)
    }

    fn comp_exp(&self) -> Self
    {
        Self::from_angle(self.y) * self.x.exp()
    }

    fn comp_ln(&self) -> Self
    {
        Self::new(self.length().ln(), self.y.atan2(self.x))
    }
    
    fn comp_sin(&self) -> Self
    {
        Self::new(self.x.sin() * self.y.cosh(), self.x.cos() * self.y.sinh())
    }
    
    fn comp_cos(&self) -> Self
    {
        Self::new(self.x.cos() * self.y.cosh(), -self.x.sin() * self.y.sinh())
    }
}
