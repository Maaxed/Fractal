use glam::{ DVec2, I64Vec2 };

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct QuadPos
{
    pub unscaled_pos: I64Vec2,
    pub exponent: i32
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
}
