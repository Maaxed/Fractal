use crate::math::*;

pub const ITERATION_COUNT: u32 = 1024;

pub fn lyapunov<S: Scalar, const L: usize>(sequence: &[bool; L], pos: Vec2<S>, iteration_count: u32) -> f32
{
    let iteration_count = iteration_count as usize;
    
    let zero = S::zero();
    let one = S::one();
    let two: S = 2.0_f32.into();
    let four = 4.0_f32.into();
    if pos.x() < zero || pos.y() < zero || pos.x() > four || pos.y() > four
    {
        return 0.0;
    }

    let mut x: S = 0.5_f32.into();
    let mut lyapunov_exp = 0.0;

    for i in 0..iteration_count
    {
        let rn = if sequence[i % L] { pos.y() } else { pos.x() };

        if i != 0
        {
            lyapunov_exp += ln(abs((rn * (one - two * x)).as_()));
        }

        x = rn * x * (one - x);
    }

    lyapunov_exp / iteration_count as f32
}