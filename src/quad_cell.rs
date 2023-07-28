use glam::{ DVec2, IVec2 };

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct QuadPos
{
    pub unscaled_pos: IVec2,
    pub exponent: i32
}

impl QuadPos
{
    pub fn cell_size(&self) -> f64
    {
        2.0_f64.powi(self.exponent)
    }

    pub fn cell_center(&self) -> DVec2
    {
        let cell_size = self.cell_size();

        (self.unscaled_pos.as_dvec2() + 0.5) * cell_size
    }
}
