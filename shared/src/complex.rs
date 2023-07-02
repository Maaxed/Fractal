use glam::{DVec2, Vec2};

pub trait Complex
{
    fn comp_conjugate(&self) -> Self;
    fn comp_mul(&self, rhs: &Self) -> Self;
    fn comp_div(&self, rhs: &Self) -> Self;
    fn comp_squared(&self) -> Self;
    //fn comp_exp(&self) -> Self;
    //fn comp_ln(&self) -> Self;
}

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

    /*fn comp_exp(&self) -> Self
    {
        Self::from_angle(self.y) * self.x.exp()
    }

    fn comp_ln(&self) -> Self
    {
        Self::new(self.length().ln(), f64::atan2(self.y, self.x))
    }*/
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

    /*fn comp_exp(&self) -> Self
    {
        Self::from_angle(self.y) * self.x.exp()
    }

    fn comp_ln(&self) -> Self
    {
        Self::new(self.length().ln(), f64::atan2(self.y, self.x))
    }*/
}
