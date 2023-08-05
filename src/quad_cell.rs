use std::cmp::Ordering;

use glam::{ DVec2, I64Vec2 };

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct QuadPos
{
    pub unscaled_pos: I64Vec2,
    pub exponent: i32
}

impl PartialOrd for QuadPos
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for QuadPos
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.exponent.cmp(&other.exponent).reverse().then(self.unscaled_pos.x.cmp(&other.unscaled_pos.x)).then(self.unscaled_pos.y.cmp(&other.unscaled_pos.y))
    }
}

impl QuadPos
{
    pub fn cell_size(&self) -> f64
    {
        2.0_f64.powi(self.exponent)
    }

    pub fn cell_bottom_left(&self) -> DVec2
    {
        let cell_size = self.cell_size();

        self.unscaled_pos.as_dvec2() * cell_size
    }

    pub fn cell_center(&self) -> DVec2
    {
        let cell_size = self.cell_size();

        (self.unscaled_pos.as_dvec2() + 0.5) * cell_size
    }

    pub fn cell_top_right(&self) -> DVec2
    {
        let cell_size = self.cell_size();

        (self.unscaled_pos.as_dvec2() + 1.0) * cell_size
    }
}
